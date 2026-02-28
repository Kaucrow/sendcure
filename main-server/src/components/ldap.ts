import fs from 'fs';
import tls from 'tls';
import { Client, type ClientOptions } from 'ldapts';
import { ldap as ldapConfig } from '@global/constants.js';


type LdapUser = {
    dn: string;
    uid?: string;
    uidNumber?: string;
    cn?: string;
    mail?: string;
    telephoneNumber?: string;
    role?: string;
};

const ROLE_GROUPS = new Set(['counter', 'dispatch', 'customer-service', 'admins']);

class LdapComponent {
    async authenticate(input: { uid: string; password: string }): Promise<LdapUser | null> {
        const ca = ldapConfig.caPath ? fs.readFileSync(ldapConfig.caPath) : undefined;
        let clientOptions: ClientOptions;
        if (ca) {
            const tlsOptions: tls.ConnectionOptions = {
                ca: [ca],
                rejectUnauthorized: ldapConfig.tlsRejectUnauthorized ?? true
            };
            clientOptions = { url: ldapConfig.url, tlsOptions };
        } else {
            clientOptions = { url: ldapConfig.url };
        }

        const client = new Client(clientOptions);

        try {
            if (ldapConfig.bindDn && ldapConfig.bindPassword) {
                await client.bind(ldapConfig.bindDn, ldapConfig.bindPassword);
            }

            const { searchEntries } = await client.search(ldapConfig.baseDn, {
                scope: 'sub',
                filter: ldapConfig.userFilter.replace('{uid}', input.uid),
                attributes: ['dn', 'uid', 'uidNumber', 'cn', 'mail', 'telephoneNumber']
            });

            const entry = searchEntries[0] as LdapUser | undefined;
            if (!entry?.dn) return null;

            await client.bind(entry.dn, input.password);

            if (ldapConfig.bindDn && ldapConfig.bindPassword) {
                await client.bind(ldapConfig.bindDn, ldapConfig.bindPassword);
            }

            const groupBaseDn = ldapConfig.groupBaseDn ?? ldapConfig.baseDn;
            let role: string | undefined;
            try {
                const { searchEntries: groupEntries } = await client.search(groupBaseDn, {
                    scope: 'sub',
                    filter: `(member=${entry.dn})`,
                    attributes: ['cn']
                });

                role = groupEntries
                    .map((group) => String((group as { cn?: string }).cn ?? '').toLowerCase())
                    .find((cn) => ROLE_GROUPS.has(cn));
            } catch {
                role = undefined;
            }

            if (role) {
                entry.role = role;
            }

            return entry;
        } catch {
            return null;
        } finally {
            try {
                await client.unbind();
            } catch {
                // ignore unbind errors
            }
        }
    }
}
export const ldap = new LdapComponent();