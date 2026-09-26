
int FUN_00410790(void)

{
  uint *puVar1;
  int local_2c;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062a578;
  local_c = ExceptionList;
  local_2c = 0;
  ExceptionList = &local_c;
  puVar1 = FUN_00597110('\x01');
  if ((puVar1 == (uint *)0x0) && (puVar1 = FUN_00597110('\x04'), puVar1 == (uint *)0x0)) {
    ExceptionList = local_c;
    return local_2c;
  }
  FUN_0051c300(local_28);
  local_4 = 0;
  FUN_0051c510(local_28,(undefined4 *)puVar1[1],*puVar1);
  FUN_0051c360(local_28,&local_2c);
  if ((undefined *)puVar1[1] != (undefined *)0x0) {
    FUN_00618b60((undefined *)puVar1[1]);
  }
  puVar1[1] = 0;
  FUN_00618b60((undefined *)puVar1);
  local_4 = 0xffffffff;
  FUN_0051c350(local_28);
  ExceptionList = local_c;
  return local_2c;
}

