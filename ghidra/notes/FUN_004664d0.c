
void __fastcall FUN_004664d0(undefined4 *param_1)

{
  undefined4 *puVar1;
  int iVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006319a0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065a1c0;
  local_4 = 4;
  if ((undefined4 *)param_1[0x57] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x57])(1);
  }
  if ((undefined4 *)param_1[0x58] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x58])(1);
  }
  puVar1 = (undefined4 *)param_1[0x59];
  if (puVar1 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar1);
    FUN_00618b60((undefined *)puVar1);
  }
  if (param_1[0x5b] != 0) {
    for (iVar2 = *(int *)(param_1[0x5b] + 8); iVar2 != 0; iVar2 = *(int *)(iVar2 + 8)) {
      *(undefined4 *)(iVar2 + 0x6c) = 1;
    }
    FUN_005f5b20((void *)param_1[0x5b]);
    if ((undefined4 *)param_1[0x5b] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x5b])(1);
    }
  }
  if ((undefined4 *)param_1[0x5a] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x5a])(1);
  }
  if ((undefined4 *)param_1[0x77] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x77])(1);
  }
  FUN_0041d1f0(0);
  local_4._0_1_ = 3;
  FUN_00601990((int)(param_1 + 0x6a));
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_00601990((int)(param_1 + 0x5d));
  param_1[0x50] = &PTR_LAB_00658518;
  param_1[0x4a] = &PTR_LAB_00658518;
  local_4 = 0xffffffff;
  FUN_00606580(param_1);
  ExceptionList = local_c;
  return;
}

