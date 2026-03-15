
void __fastcall FUN_0046cd20(undefined4 *param_1)

{
  undefined *puVar1;
  undefined4 *puVar2;
  int iVar3;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  puStack_8 = &LAB_00632036;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_0065a270;
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
  if (param_1[0x5e] != 0) {
    for (iVar3 = *(int *)(param_1[0x5e] + 8); iVar3 != 0; iVar3 = *(int *)(iVar3 + 8)) {
      *(undefined4 *)(iVar3 + 0x6c) = 1;
    }
    FUN_005f5b20((void *)param_1[0x5e]);
    if ((undefined4 *)param_1[0x5e] != (undefined4 *)0x0) {
      (*(code *)**(undefined4 **)param_1[0x5e])(1);
    }
  }
  if ((HDC)param_1[0x58] != (HDC)0x0) {
    DeleteDC((HDC)param_1[0x58]);
    param_1[0x58] = 0;
  }
  puVar2 = param_1 + 0x59;
  iVar3 = 5;
  do {
    if ((HGDIOBJ)*puVar2 != (HGDIOBJ)0x0) {
      DeleteObject((HGDIOBJ)*puVar2);
    }
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
    iVar3 = iVar3 + -1;
  } while (iVar3 != 0);
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_004ac2f0(param_1);
  ExceptionList = pvStack_c;
  return;
}

