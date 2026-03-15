
undefined4 __thiscall FUN_005093e0(void *this,undefined4 *param_1)

{
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641958;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = 0;
  FUN_004f25c0(local_2c,this,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_004f26c0(local_2c);
  ExceptionList = pvStack_c;
  return 1;
}

