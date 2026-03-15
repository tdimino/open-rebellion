
void * __thiscall FUN_005f3e70(void *this,void *param_1)

{
  char *pcVar1;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0065562b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f3dc0(local_18,(int)this);
  pcVar1 = *(char **)((int)this + 4);
  local_4 = 1;
  if ((pcVar1 != (char *)0x0) && (*pcVar1 != '\0')) {
    *(char **)((int)this + 4) = pcVar1 + 1;
  }
  FUN_005f3dc0(param_1,(int)local_18);
  local_4 = local_4 & 0xffffff00;
  FUN_005f3e00(local_18);
  ExceptionList = local_c;
  return param_1;
}

