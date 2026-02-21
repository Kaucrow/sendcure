export class DbError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'DatabaseError';
  }
}

export class DbConflictError extends DbError {
  constructor(message: string, public detail?: string) {
    super(message);
    this.name = 'ConflictError';
  }
}

export class DbNotNullViolationError extends DbError {
  constructor(message: string, public detail?: string) {
    super(message);
    this.name = 'NotNullViolationError';
  }
}

export class DbSchemaValidationError extends DbError {
  constructor(message: string, public rawData?: any) {
    super(message);
    this.name = 'SchemaValidationError';
  }
}