
void * __thiscall FUN_00411270(void *this,void *param_1)

{
  BOOL BVar1;
  undefined2 local_124;
  undefined2 local_122;
  undefined4 local_120;
  DWORD local_11c;
  undefined4 local_118 [3];
  CHAR local_10c [256];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a605;
  pvStack_c = ExceptionList;
  local_120 = 0;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_118);
  local_122 = *(undefined2 *)((int)this + 0x94);
  local_4 = 1;
  local_124 = 0x110c;
  FUN_005f3010(local_118,&local_124);
  local_11c = 0xff;
  BVar1 = GetUserNameA(local_10c,&local_11c);
  if (BVar1 != 0) {
    FUN_005f35e0(local_118,local_10c);
  }
  FUN_005f2f90(param_1,(int)local_118);
  local_120 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_005f2ff0(local_118);
  ExceptionList = pvStack_c;
  return param_1;
}

