
void __fastcall FUN_005d6c10(int *param_1)

{
  int *piVar1;
  int iVar2;
  undefined4 *puVar3;
  int **ppiVar4;
  int *piStack_ac;
  tagRECT local_7c;
  undefined4 auStack_6c [18];
  undefined4 uStack_24;
  undefined4 uStack_14;
  undefined4 uStack_4;
  
  GetClientRect((HWND)param_1[6],&local_7c);
  if (param_1[0x51] != 0) {
    puVar3 = auStack_6c;
    for (iVar2 = 0x1b; iVar2 != 0; iVar2 = iVar2 + -1) {
      *puVar3 = 0;
      puVar3 = puVar3 + 1;
    }
    auStack_6c[0] = 0x6c;
    auStack_6c[1] = 1;
    uStack_4 = 0x2200;
    DAT_006bcbb4 = (**(code **)(*(int *)param_1[0x53] + 0x18))();
    if (DAT_006bcbb4 != 0) {
      piStack_ac = (int *)0x5d6c8e;
      FUN_005d93f0(0,DAT_006bcbb4);
    }
    piStack_ac = (int *)param_1[0x53];
    piVar1 = param_1 + 0x56;
    local_7c.top = 7;
    uStack_14 = 0x2840;
    DAT_006bcbb4 = (**(code **)(*piStack_ac + 0x18))();
    if (DAT_006bcbb4 != 0) {
      FUN_005d93f0(0,DAT_006bcbb4);
    }
    puVar3 = (undefined4 *)&stack0xffffff74;
    for (iVar2 = 0x1b; iVar2 != 0; iVar2 = iVar2 + -1) {
      *puVar3 = 0;
      puVar3 = puVar3 + 1;
    }
    local_7c.right = 0x10;
    uStack_24 = 0x20800;
    DAT_006bcbb4 = (**(code **)(*(int *)param_1[0x53] + 0x18))
                             ((int *)param_1[0x53],&stack0xffffff74,param_1 + 0x57,0);
    if (DAT_006bcbb4 != 0) {
      FUN_005d93f0(0,DAT_006bcbb4);
    }
    DAT_006bcbb4 = (**(code **)(*(int *)*piVar1 + 0xc))((int *)*piVar1,param_1[0x57]);
    if (DAT_006bcbb4 != 0) {
      FUN_005d93f0(0,DAT_006bcbb4);
    }
    DAT_006bcbb4 = (**(code **)(*(int *)param_1[0x55] + 0x70))((int *)param_1[0x55],param_1[0x3b]);
    if (DAT_006bcbb4 != 0) {
      FUN_005d93f0(0,DAT_006bcbb4);
    }
    ppiVar4 = &piStack_ac;
    for (iVar2 = 0x1b; iVar2 != 0; iVar2 = iVar2 + -1) {
      *ppiVar4 = (int *)0x0;
      ppiVar4 = ppiVar4 + 1;
    }
    piStack_ac = (int *)0x6c;
    DAT_006bcbb4 = (**(code **)(*(int *)*piVar1 + 100))((int *)*piVar1,0,&piStack_ac,1,0);
    if (DAT_006bcbb4 != 0) {
      FUN_005d93f0(0,DAT_006bcbb4);
    }
    (**(code **)(*(int *)*piVar1 + 0x80))((int *)*piVar1,0);
    (**(code **)(*param_1 + 0x28))((uint)local_7c.right < 0x10);
  }
  return;
}

