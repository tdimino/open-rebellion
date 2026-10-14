
void __thiscall FUN_00499760(int param_1,int param_2,uint *param_3,uint param_4,int param_5)

{
  undefined4 uVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  undefined4 *puVar5;
  bool bVar6;
  uint uVar7;
  
  if (param_2 != 0x100) {
    return;
  }
  bVar6 = false;
  piVar3 = (int *)0x0;
  if ((0x8f < *param_3 >> 0x18) && (*param_3 >> 0x18 < 0x98)) {
    bVar6 = true;
  }
  FUN_00619730();
  if (bVar6) {
    piVar3 = FUN_004f3220(*(int *)(param_1 + 0x20),param_3);
    FUN_004f26d0((void *)(param_1 + 0x5c),param_3);
    if (((piVar3 == (int *)0x0) || (((uint)piVar3[9] >> 6 & 3) != param_4)) ||
       (uVar1 = 1, (piVar3[0x22] & 0x2000U) == 0)) {
      uVar1 = 0;
    }
    *(undefined4 *)(param_1 + 0x40) = uVar1;
  }
  iVar4 = 0;
  if (*(int *)(param_1 + 0x40) == 0) {
    return;
  }
  if (piVar3 == (int *)0x0) {
    return;
  }
  bVar6 = false;
  *(uint *)(param_1 + 100) = param_4;
  if (param_4 == 1) {
    iVar4 = FUN_004f3dd0(*(int *)(param_1 + 0x20),1);
    bVar6 = *(int *)(param_1 + 0x20) == 1;
  }
  else if (param_4 == 2) {
    iVar4 = FUN_004f3dd0(*(int *)(param_1 + 0x20),2);
    bVar6 = *(int *)(param_1 + 0x20) == 2;
  }
  if (param_5 == 1) {
    iVar2 = *(int *)(param_1 + 0x20);
    uVar7 = 1;
  }
  else {
    if (param_5 != 2) {
      iVar2 = 0;
      goto LAB_00499887;
    }
    iVar2 = *(int *)(param_1 + 0x20);
    uVar7 = 2;
  }
  iVar2 = FUN_004f3dd0(iVar2,uVar7);
LAB_00499887:
  if (iVar4 == 0) {
    *(undefined2 *)(param_1 + 0x30) = 0x451;
    if (iVar2 == 0) {
      puVar5 = (undefined4 *)0x0;
    }
    else {
      puVar5 = (undefined4 *)(iVar2 + 0x30);
    }
    FUN_0060b9d0((void *)(param_1 + 0x44),0x703a,piVar3 + 0xc,puVar5,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (iVar2 == 0) {
      puVar5 = (undefined4 *)0x0;
    }
    else {
      puVar5 = (undefined4 *)(iVar2 + 0x30);
    }
    uVar7 = 0x703d;
  }
  else {
    *(undefined2 *)(param_1 + 0x30) = 0x450;
    if (bVar6) {
      puVar5 = (undefined4 *)(iVar4 + 0x30);
      *(undefined4 *)(param_1 + 0x28) = 1;
      FUN_0060b9d0((void *)(param_1 + 0x44),0x7038,piVar3 + 0xc,puVar5,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      uVar7 = 0x703b;
    }
    else {
      puVar5 = (undefined4 *)(iVar4 + 0x30);
      *(undefined4 *)(param_1 + 0x28) = 2;
      FUN_0060b9d0((void *)(param_1 + 0x44),0x7039,piVar3 + 0xc,puVar5,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      uVar7 = 0x703c;
    }
  }
  FUN_0060b9d0((void *)(param_1 + 0x50),uVar7,piVar3 + 0xc,puVar5,(undefined4 *)0x0,
               (undefined4 *)0x0);
  *(undefined2 *)(param_1 + 0x2e) = 0x3ed;
  return;
}

