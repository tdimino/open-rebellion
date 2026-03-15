
void __fastcall FUN_004601a0(undefined4 *param_1)

{
  undefined *puVar1;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00630fe6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065a000;
  local_4 = 1;
  if ((undefined4 *)param_1[0x51] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x51])(1);
  }
  if ((undefined4 *)param_1[0x52] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x52])(1);
  }
  if ((undefined4 *)param_1[0x53] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x53])(1);
  }
  puVar1 = (undefined *)param_1[0x55];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  if ((void *)param_1[0x57] != (void *)0x0) {
    FUN_005f5b20((void *)param_1[0x57]);
    if ((undefined4 *)param_1[0x57] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x57])(1);
    }
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ac2f0(param_1);
  ExceptionList = local_c;
  return;
}

