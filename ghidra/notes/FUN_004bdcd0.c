
undefined4 __fastcall FUN_004bdcd0(int param_1)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  uint uVar4;
  void *pvVar5;
  int *piVar6;
  uint *puVar7;
  void *pvVar8;
  undefined4 *puVar9;
  uint local_48;
  undefined4 local_44;
  undefined1 local_40 [4];
  uint local_3c;
  uint local_38;
  undefined1 local_34 [4];
  undefined1 *local_30;
  undefined1 *local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00639e58;
  local_c = ExceptionList;
  local_44 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_48);
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xfffffffb;
  uVar4 = *(uint *)(param_1 + 0x74) >> 0x18;
  local_2c = (undefined1 *)0x80;
  local_28 = 0x90;
  if ((uVar4 < 0x80) || (0x8f < uVar4)) {
    bVar3 = false;
  }
  else {
    bVar3 = true;
  }
  FUN_00619730();
  if (bVar3) {
    uVar4 = *(uint *)(param_1 + 0x38) >> 0x18;
    local_2c = (undefined1 *)0x90;
    local_28 = 0x98;
    if ((uVar4 < 0x90) || (0x97 < uVar4)) {
      bVar3 = false;
    }
    else {
      bVar3 = true;
    }
    FUN_00619730();
    if (bVar3) {
      iVar1 = *(int *)(param_1 + 0x68);
      local_30 = &stack0xffffffa4;
      FUN_004f26d0(&stack0xffffffa4,(undefined4 *)(param_1 + 0x38));
      pvVar5 = FUN_00403d30((void *)(iVar1 + 0x2c));
      if (pvVar5 != (void *)0x0) {
        piVar6 = FUN_004bd7d0(pvVar5,local_40);
        iVar1 = *piVar6;
        iVar2 = *(int *)(param_1 + 0x74);
        FUN_00619730();
        if (iVar1 == iVar2) {
          if (((*(byte *)((int)pvVar5 + 0x30) & 1) == 0) ||
             ((*(byte *)((int)pvVar5 + 0x28) & 2) != 0)) {
LAB_004bddfd:
            FUN_004ece80((undefined4 *)(param_1 + 0x38));
          }
          else {
            uVar4 = *(uint *)(param_1 + 0x20) & 0xffffffbf;
            *(uint *)(param_1 + 0x20) = uVar4;
            *(uint *)(param_1 + 0x20) = uVar4 | 4;
          }
        }
        else if ((*(byte *)(param_1 + 0x20) & 0x40) != 0) {
          if (((*(byte *)((int)pvVar5 + 0x30) & 1) == 0) ||
             ((*(byte *)((int)pvVar5 + 0x28) & 2) != 0)) goto LAB_004bddfd;
          *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 4;
        }
      }
    }
    if ((*(byte *)(param_1 + 0x20) & 4) == 0) {
      for (pvVar5 = (void *)thunk_FUN_005f5060(param_1 + 0x3c); pvVar5 != (void *)0x0;
          pvVar5 = *(void **)((int)pvVar5 + 0x10)) {
        if ((*(byte *)(param_1 + 0x20) & 4) != 0) goto LAB_004be1f1;
        puVar7 = FUN_00403040(pvVar5,local_40);
        local_4._0_1_ = 1;
        pvVar8 = FUN_004f5940(*(void **)(param_1 + 100),puVar7);
        local_4._0_1_ = 0;
        FUN_00619730();
        if (pvVar8 != (void *)0x0) {
          FUN_004f26d0(&local_48,(undefined4 *)((int)pvVar8 + 0x38));
          local_24 = 0x90;
          local_20 = 0x98;
          if ((local_48 >> 0x18 < 0x90) || (0x97 < local_48 >> 0x18)) {
            bVar3 = false;
          }
          else {
            bVar3 = true;
          }
          FUN_00619730();
          if ((!bVar3) &&
             (piVar6 = FUN_004f2d10(*(int *)(param_1 + 0x24),&local_48), piVar6 != (int *)0x0)) {
            do {
              puVar7 = FUN_004025b0(piVar6,&local_3c);
              local_1c = 0x90;
              local_18 = 0x98;
              local_4._0_1_ = 2;
              if ((*puVar7 >> 0x18 < 0x90) || (0x97 < *puVar7 >> 0x18)) {
                bVar3 = false;
              }
              else {
                bVar3 = true;
              }
              FUN_00619730();
              local_4._0_1_ = 0;
              FUN_00619730();
            } while ((!bVar3) && (piVar6 = (int *)piVar6[7], piVar6 != (int *)0x0));
            if (piVar6 != (int *)0x0) {
              puVar7 = FUN_004025b0(piVar6,&local_38);
              local_4._0_1_ = 3;
              FUN_004f26d0(&local_48,puVar7);
              local_4._0_1_ = 0;
              FUN_00619730();
            }
          }
          local_14 = 0x90;
          local_10 = 0x98;
          if ((local_48 >> 0x18 < 0x90) || (0x97 < local_48 >> 0x18)) {
            bVar3 = false;
          }
          else {
            bVar3 = true;
          }
          FUN_00619730();
          if (bVar3) {
            iVar1 = *(int *)(param_1 + 0x68);
            local_2c = &stack0xffffffa4;
            FUN_004f26d0(&stack0xffffffa4,&local_48);
            pvVar8 = FUN_00403d30((void *)(iVar1 + 0x2c));
            if (pvVar8 != (void *)0x0) {
              piVar6 = FUN_004bd7d0(pvVar8,local_34);
              iVar1 = *piVar6;
              iVar2 = *(int *)(param_1 + 0x74);
              FUN_00619730();
              if (((iVar1 == iVar2) && ((*(byte *)((int)pvVar8 + 0x30) & 1) != 0)) &&
                 ((*(byte *)((int)pvVar8 + 0x28) & 2) == 0)) {
                uVar4 = *(uint *)(param_1 + 0x20) & 0xffffffbf;
                *(uint *)(param_1 + 0x20) = uVar4;
                *(uint *)(param_1 + 0x20) = uVar4 | 4;
                puVar9 = FUN_00403040(pvVar8,&local_30);
                local_4._0_1_ = 4;
                FUN_004f26d0((void *)(param_1 + 0x38),puVar9);
                local_4._0_1_ = 0;
                FUN_00619730();
              }
            }
          }
        }
      }
      if ((*(byte *)(param_1 + 0x20) & 4) == 0) {
        local_14 = 0x80;
        *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xffffffbf;
        uVar4 = *(uint *)(param_1 + 0x74) >> 0x18;
        local_10 = 0x90;
        if ((uVar4 < 0x80) || (0x8f < uVar4)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        FUN_00619730();
        if (bVar3) {
          pvVar8 = FUN_0041a460(*(void **)(param_1 + 0x68),(uint *)(param_1 + 0x74),0,0,1,2,0,0,
                                (void *)0x1);
          pvVar5 = (void *)(param_1 + 0x54);
          FUN_00435790(pvVar5,(int)pvVar8);
          puVar9 = FUN_004357b0(pvVar5,&local_30);
          local_4._0_1_ = 5;
          FUN_004f26d0((void *)(param_1 + 0x38),puVar9);
          local_4._0_1_ = 0;
          FUN_00619730();
          FUN_005f58b0(pvVar5);
        }
        local_14 = 0x90;
        uVar4 = *(uint *)(param_1 + 0x38) >> 0x18;
        local_10 = 0x98;
        if ((uVar4 < 0x90) || (0x97 < uVar4)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        FUN_00619730();
        if (bVar3) {
          *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) | 4;
        }
        if ((*(byte *)(param_1 + 0x20) & 4) == 0) {
          local_14 = 0x80;
          *(uint *)(param_1 + 0x20) = *(uint *)(param_1 + 0x20) & 0xffffffbf;
          uVar4 = *(uint *)(param_1 + 0x74) >> 0x18;
          local_10 = 0x90;
          if ((uVar4 < 0x80) || (0x8f < uVar4)) {
            bVar3 = false;
          }
          else {
            bVar3 = true;
          }
          FUN_00619730();
          if (bVar3) {
            puVar9 = FUN_0041b2b0(*(void **)(param_1 + 0x68),&local_30,(uint *)(param_1 + 0x74),0,0,
                                  1,0,0,0x200);
            local_4._0_1_ = 6;
            FUN_004f26d0((uint *)(param_1 + 0x38),puVar9);
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_00619730();
            local_14 = 0x90;
            uVar4 = *(uint *)(param_1 + 0x38) >> 0x18;
            local_10 = 0x98;
            if ((uVar4 < 0x90) || (0x97 < uVar4)) {
              bVar3 = false;
            }
            else {
              bVar3 = true;
            }
            FUN_00619730();
            if (bVar3) {
              uVar4 = *(uint *)(param_1 + 0x20);
              *(uint *)(param_1 + 0x20) = uVar4 | 4;
              *(uint *)(param_1 + 0x20) = uVar4 | 0x44;
            }
          }
          goto LAB_004be1eb;
        }
      }
    }
  }
  else {
LAB_004be1eb:
    if ((*(byte *)(param_1 + 0x20) & 4) == 0) goto LAB_004be1f9;
  }
LAB_004be1f1:
  local_44 = 1;
LAB_004be1f9:
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_44;
}

