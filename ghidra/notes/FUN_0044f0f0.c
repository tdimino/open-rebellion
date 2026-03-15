
void __fastcall FUN_0044f0f0(undefined4 *param_1)

{
  undefined *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0062f968;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_00659cd0;
  puVar1 = (undefined *)param_1[0x48];
  local_4 = 0;
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  if ((undefined4 *)param_1[0x47] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x47])(1);
  }
  if ((undefined4 *)param_1[0x46] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x46])(1);
  }
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = local_c;
  return;
}

