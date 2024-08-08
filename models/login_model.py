from typing import Optional

from pydantic import BaseModel


class CasLoginRequest(BaseModel):
    username: str
    password: str


class CasLoginResponse(BaseModel):
    status: str
    message: str
    authToken: Optional[str] = None
