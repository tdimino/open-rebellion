
void __fastcall FUN_00463650(undefined4 *param_1)

{
  undefined *puVar1;
  int iVar2;
  undefined4 *puVar3;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00631536;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_0065a130;
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
  if ((undefined4 *)param_1[0x54] != (undefined4 *)0x0) {
    (*(code *)**(undefined4 **)param_1[0x54])(1);
  }
  puVar1 = (undefined *)param_1[0x56];
  if (puVar1 != (undefined *)0x0) {
    FUN_00601990((int)puVar1);
    FUN_00618b60(puVar1);
  }
  if (param_1[0x5c] != 0) {
    for (iVar2 = *(int *)(param_1[0x5c] + 8); iVar2 != 0; iVar2 = *(int *)(iVar2 + 8)) {
      *(undefined4 *)(iVar2 + 0x6c) = 1;
    }
    FUN_005f5b20((void *)param_1[0x5c]);
    if ((undefined4 *)param_1[0x5c] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x5c])(1);
    }
  }
  if (param_1[0x5d] != 0) {
    for (iVar2 = *(int *)(param_1[0x5d] + 8); iVar2 != 0; iVar2 = *(int *)(iVar2 + 8)) {
      *(undefined4 *)(iVar2 + 0x6c) = 1;
    }
    FUN_005f5b20((void *)param_1[0x5d]);
    if ((undefined4 *)param_1[0x5d] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x5d])(1);
    }
  }
  puVar3 = param_1 + 0x57;
  iVar2 = 5;
  do {
    if ((HGDIOBJ)*puVar3 != (HGDIOBJ)0x0) {
      DeleteObject((HGDIOBJ)*puVar3);
    }
    puVar3 = puVar3 + 1;
    iVar2 = iVar2 + -1;
  } while (iVar2 != 0);
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ac2f0(param_1);
  ExceptionList = local_c;
  return;
}

