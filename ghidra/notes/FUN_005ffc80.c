
ATOM __thiscall FUN_005ffc80(void *this,UINT param_1)

{
  ATOM AVar1;
  WNDCLASSA local_28;
  
  local_28.hInstance = *(HINSTANCE *)((int)this + 0x1c);
  local_28.hIcon = *(HICON *)((int)this + 0x60);
  local_28.style = param_1;
  local_28.hCursor = *(HCURSOR *)((int)this + 100);
  local_28.cbClsExtra = 0;
  local_28.hbrBackground = *(HBRUSH *)((int)this + 0x5c);
  local_28.lpszMenuName = (LPCSTR)0x0;
  local_28.lpszClassName = *(LPCSTR *)((int)this + 0x48);
  local_28.lpfnWndProc = FUN_00600310;
  local_28.cbWndExtra = 4;
  AVar1 = RegisterClassA(&local_28);
  return AVar1;
}

