
void __fastcall FUN_00442f40(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_0062ec1e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00659c18;
  puVar1 = (undefined4 *)param_1[0x48];
  local_4 = 5;
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  if ((void *)param_1[0x47] != (void *)0x0) {
    FUN_005f5b20((void *)param_1[0x47]);
    if ((undefined4 *)param_1[0x47] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x47])(1);
    }
  }
  local_4._0_1_ = 4;
  FUN_00601990((int)(param_1 + 0x66));
  local_4._0_1_ = 3;
  FUN_00601990((int)(param_1 + 0x59));
  local_4._0_1_ = 2;
  FUN_00601990((int)(param_1 + 0x4c));
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = local_c;
  return;
}

