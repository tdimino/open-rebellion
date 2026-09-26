
void __fastcall FUN_004a10a0(int *param_1)

{
  bool bVar1;
  int *piVar2;
  int iVar3;
  uint *puVar4;
  void *pvVar5;
  uint uVar6;
  undefined4 *puVar7;
  undefined4 uVar8;
  uint *puVar9;
  int iVar10;
  undefined *puVar11;
  char *pcVar12;
  uint uVar13;
  undefined1 uVar14;
  uint *puVar15;
  undefined4 local_28;
  void *local_24;
  undefined4 local_20;
  tagRECT local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006376d9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = FUN_004a25c0(param_1);
  if (piVar2 != (int *)0x0) {
    puVar9 = (uint *)(param_1 + 0x72);
    iVar3 = FUN_004ece60(puVar9);
    if (iVar3 != 0) {
      puVar4 = FUN_004025b0(piVar2,&local_28);
      uVar13 = *puVar9;
      uVar6 = *puVar4;
      FUN_00619730();
      if (uVar13 != uVar6) {
        pvVar5 = FUN_004f5940(param_1 + 0x45,puVar9);
        if (pvVar5 != (void *)0x0) {
          FUN_004f58e0(param_1 + 0x45,puVar9);
        }
      }
    }
    FUN_004ece80(puVar9);
    puVar7 = (undefined4 *)param_1[0x5c];
    if (puVar7 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar7);
      FUN_00618b60((undefined *)puVar7);
      param_1[0x5c] = 0;
    }
    if (param_1[0x71] == 0) {
      *(undefined4 *)(param_1[0x5d] + 0x30) = 1;
      *(undefined4 *)(param_1[0x5e] + 0x30) = 1;
    }
    else {
      if (*(int *)(param_1[0x53] + 0x9c) == 1) {
        uVar13 = 1;
      }
      else if (*(int *)(param_1[0x53] + 0x9c) == 2) {
        uVar13 = 2;
      }
      else {
        uVar13 = 0;
      }
      uVar6 = FUN_004a24b0(param_1);
      if (uVar6 == uVar13) {
        puVar4 = FUN_0042d170((void *)param_1[0x71],&local_28);
        local_4 = 0;
        piVar2 = FUN_004f3000(*(int *)(param_1[0x53] + 0x9c),puVar4);
        local_4 = 0xffffffff;
        FUN_00619730();
        if (piVar2 != (int *)0x0) {
          FUN_004f26d0(puVar9,piVar2 + 0x1c);
        }
      }
      else {
        puVar4 = FUN_0042d170((void *)param_1[0x71],&local_28);
        local_24 = (void *)0x50;
        local_20 = 0x60;
        bVar1 = true;
        local_4 = 1;
        if ((*puVar4 >> 0x18 < 0x50) || (0x5f < *puVar4 >> 0x18)) {
          bVar1 = false;
        }
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        if (bVar1) {
          puVar4 = FUN_004025b0(piVar2,&local_28);
          local_4 = 2;
          FUN_004f26d0(puVar9,puVar4);
          local_4 = 0xffffffff;
          FUN_00619730();
        }
      }
      puVar4 = FUN_004ece40(puVar9);
      piVar2 = (int *)0x0;
      if (puVar4 != (uint *)0x0) {
        local_28 = (int *)FUN_00618b70(0x28);
        local_4 = 3;
        if (local_28 == (void *)0x0) {
          puVar7 = (undefined4 *)0x0;
        }
        else {
          puVar7 = FUN_004acd80(local_28,puVar9,0);
        }
        local_4 = 0xffffffff;
        FUN_004acba0(param_1 + 0x45,puVar7,0);
        local_28 = FUN_004f2d10(*(int *)(param_1[0x53] + 0x9c),puVar9);
        piVar2 = local_28;
        if (local_28 != (int *)0x0) {
          local_24 = (void *)0x90;
          local_20 = 0x98;
          if ((*puVar9 >> 0x18 < 0x90) || (0x97 < *puVar9 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            uVar8 = FUN_006037f0(7);
            pvVar5 = (void *)FUN_00618b70(0x28);
            local_4 = 4;
            local_24 = pvVar5;
            if (pvVar5 == (void *)0x0) {
              puVar9 = (uint *)0x0;
              local_4 = 0xffffffff;
            }
            else {
              uVar14 = 2;
              iVar3 = FUN_00509610((int)local_28);
              uVar13 = FUN_0045c970(iVar3);
              puVar9 = FUN_005fbd20(pvVar5,uVar8,uVar13,uVar14);
              local_4 = 0xffffffff;
            }
          }
          else {
            local_24 = (void *)0x30;
            local_20 = 0x40;
            if ((*puVar9 >> 0x18 < 0x30) || (0x3f < *puVar9 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (!bVar1) {
              iVar3 = param_1[0x6c];
            }
            else {
              iVar3 = param_1[0x6c];
            }
            puVar9 = FUN_0042c3b0(iVar3,puVar9,local_28,(uint)!bVar1,1);
          }
          piVar2 = local_28;
          if (puVar9 != (uint *)0x0) {
            puVar4 = (uint *)FUN_006073d0(param_1);
            pvVar5 = (void *)FUN_00618b70(0x28);
            local_4 = 5;
            local_24 = pvVar5;
            if (pvVar5 == (void *)0x0) {
              pvVar5 = (void *)0x0;
            }
            else {
              puVar15 = puVar4;
              iVar3 = FUN_005fc0f0((int *)puVar9);
              iVar10 = FUN_005fc0e0((int *)puVar9);
              pvVar5 = FUN_005fbda0(pvVar5,iVar10,iVar3,puVar15);
            }
            local_4 = 0xffffffff;
            param_1[0x5c] = (int)pvVar5;
            iVar3 = FUN_005fc0e0((int *)puVar9);
            iVar10 = FUN_005fc0f0((int *)puVar9);
            FUN_005fcc30(puVar4,(uint *)param_1[0x5c],0,0,(0x7a - iVar3) / 2 + 0x6c,
                         (0x32 - iVar10) / 2 + 0x25,(undefined4 *)0x0,0);
            FUN_005fd0f0(puVar9,(uint *)param_1[0x5c],0,0);
            FUN_005fbfa0(puVar9);
            FUN_00618b60((undefined *)puVar9);
            piVar2 = local_28;
          }
        }
      }
      *(undefined4 *)(param_1[0x5d] + 0x30) = 0;
      *(undefined4 *)(param_1[0x5e] + 0x30) = 0;
      if (piVar2 == (int *)0x0) {
        local_28 = (int *)CONCAT22(DAT_0065d424,0x8527);
        FUN_006019a0((void *)param_1[0x5d],&local_28);
      }
      else {
        puVar11 = FUN_004f62d0((int)piVar2);
        pcVar12 = (char *)FUN_00583c40((int)puVar11);
        FUN_00601aa0((void *)param_1[0x5d],pcVar12);
      }
      local_28 = (int *)CONCAT22(DAT_0065d424,0x8521);
      FUN_006019a0((void *)param_1[0x5e],&local_28);
      FUN_00601c90((void *)param_1[0x5e],0xffffff,(HWND)0x0);
    }
    SetRect(&local_1c,0x6c,0x25,0xe6,0x57);
    InvalidateRect((HWND)param_1[6],&local_1c,0);
    FUN_00601dc0((void *)param_1[0x5e],(HWND)param_1[6],0);
    FUN_00601dc0((void *)param_1[0x5d],(HWND)param_1[6],0);
  }
  ExceptionList = local_c;
  return;
}

