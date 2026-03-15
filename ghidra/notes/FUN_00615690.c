
undefined4 __fastcall FUN_00615690(void *param_1)

{
  uint uVar1;
  int iVar2;
  undefined4 *puVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  undefined4 *puVar9;
  uint uVar10;
  undefined4 *puVar11;
  
  uVar7 = *(uint *)((int)param_1 + 0x50);
  uVar10 = uVar7;
  if ((int)uVar7 < 2) {
    uVar10 = 1;
  }
  uVar6 = *(uint *)((int)param_1 + 0x14);
  uVar1 = *(uint *)((int)param_1 + 0x10);
  if (uVar1 < uVar6) {
    iVar4 = uVar6 - uVar1;
  }
  else {
    iVar4 = 0;
  }
  if ((int)uVar7 <= (int)(iVar4 + uVar10)) {
    if ((int)uVar7 < 2) {
      uVar7 = 1;
    }
    if (uVar1 < uVar6) {
      iVar4 = uVar6 - uVar1;
    }
    else {
      iVar4 = 0;
    }
    uVar7 = iVar4 + uVar7;
  }
  iVar4 = 0;
  if (*(code **)((int)param_1 + 0x5c) == (code *)0x0) {
    puVar3 = (undefined4 *)FUN_00618b70(uVar7);
  }
  else {
    puVar3 = (undefined4 *)(**(code **)((int)param_1 + 0x5c))();
  }
  if (puVar3 != (undefined4 *)0x0) {
    puVar11 = *(undefined4 **)((int)param_1 + 0x14);
    puVar9 = *(undefined4 **)((int)param_1 + 0x10);
    if (puVar9 < puVar11) {
      iVar5 = (int)puVar11 - (int)puVar9;
    }
    else {
      iVar5 = 0;
    }
    if (iVar5 != 0) {
      if (puVar9 < puVar11) {
        uVar10 = (int)puVar11 - (int)puVar9;
      }
      else {
        uVar10 = 0;
      }
      puVar11 = puVar3;
      for (uVar6 = uVar10 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *puVar11 = *puVar9;
        puVar9 = puVar9 + 1;
        puVar11 = puVar11 + 1;
      }
      for (uVar10 = uVar10 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
        *(undefined1 *)puVar11 = *(undefined1 *)puVar9;
        puVar9 = (undefined4 *)((int)puVar9 + 1);
        puVar11 = (undefined4 *)((int)puVar11 + 1);
      }
      puVar9 = *(undefined4 **)((int)param_1 + 0x10);
      iVar4 = (int)puVar3 - (int)puVar9;
    }
    if (*(code **)((int)param_1 + 0x60) == (code *)0x0) {
      FUN_00618b60((undefined *)puVar9);
    }
    else {
      (**(code **)((int)param_1 + 0x60))();
    }
    FUN_00618020(param_1,puVar3,(undefined1 *)(uVar7 + (int)puVar3),0);
    if ((iVar4 != 0) && (*(int *)((int)param_1 + 0x2c) != 0)) {
      *(int *)((int)param_1 + 0x24) = *(int *)((int)param_1 + 0x24) + iVar4;
      *(int *)((int)param_1 + 0x28) = *(int *)((int)param_1 + 0x28) + iVar4;
      *(int *)((int)param_1 + 0x2c) = iVar4 + *(int *)((int)param_1 + 0x2c);
      *(undefined4 *)((int)param_1 + 0xc) = 0xffffffff;
    }
    if (*(int *)((int)param_1 + 0x20) != 0) {
      iVar5 = *(int *)((int)param_1 + 0x18);
      iVar2 = *(int *)((int)param_1 + 0x1c);
      iVar8 = *(int *)((int)param_1 + 0x20) + iVar4;
      iVar4 = iVar5 + iVar4;
      *(int *)((int)param_1 + 0x20) = iVar8;
      *(int *)((int)param_1 + 0x18) = iVar4;
      *(int *)((int)param_1 + 0x1c) = iVar4;
      if (iVar8 != 0) {
        *(int *)((int)param_1 + 0x1c) = iVar4 + (iVar2 - iVar5);
      }
    }
    return 1;
  }
  return 0xffffffff;
}

