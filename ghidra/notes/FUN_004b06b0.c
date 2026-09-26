
int * __thiscall FUN_004b06b0(void *this,undefined4 *param_1)

{
  uint uVar1;
  bool bVar2;
  void *pvVar3;
  int iVar4;
  void *pvVar5;
  uint *puVar6;
  void *pvVar7;
  int *piVar8;
  void *pvVar9;
  undefined4 *puVar10;
  undefined3 extraout_var;
  uint uVar11;
  int *local_70;
  void *local_6c;
  uint local_68;
  uint local_64;
  uint local_60;
  int local_5c;
  undefined1 local_58 [4];
  undefined1 local_54 [4];
  uint local_50;
  undefined1 local_4c [4];
  undefined1 local_48 [4];
  undefined1 local_44 [4];
  undefined4 *local_40;
  void *local_3c;
  undefined4 local_38;
  undefined4 local_34 [2];
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00638e59;
  pvStack_c = ExceptionList;
  uVar11 = 0;
  local_60 = 0;
  local_70 = (int *)0x0;
  ExceptionList = &pvStack_c;
  local_6c = this;
  FUN_004ece30(&local_64);
  local_4 = 0;
  FUN_004ece30(&local_68);
  local_4._0_1_ = 1;
  FUN_004f4340(local_34);
  local_4._0_1_ = 2;
  *param_1 = 0;
  FUN_004f26d0(&local_64,(undefined4 *)(*(int *)((int)this + 0x10) + 0x38));
  iVar4 = *(int *)((int)this + 0xc);
  local_40 = (undefined4 *)&stack0xffffff7c;
  FUN_004f26d0(&stack0xffffff7c,(undefined4 *)((int)this + 0x28));
  pvVar3 = FUN_00419e40((void *)(iVar4 + 0x78));
  if (pvVar3 == (void *)0x0) goto LAB_004b0b35;
  local_3c = (void *)0x14;
  local_38 = 0x1c;
  if ((local_64 >> 0x18 < 0x14) || (0x1b < local_64 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if ((bVar2) && (iVar4 = FUN_004333f0(pvVar3,(int *)&local_64), iVar4 != 0)) {
    uVar1 = *(uint *)(*(int *)((int)local_6c + 0x10) + 0x60);
    if ((uVar1 & 0x1000000) != 0) {
      *param_1 = 1;
      goto LAB_004b07b1;
    }
    if ((uVar1 & 0x4000000) == 0) goto LAB_004b07b1;
    iVar4 = *(int *)((int)local_6c + 0xc);
    local_40 = (undefined4 *)&stack0xffffff7c;
    FUN_004f26d0(&stack0xffffff7c,(undefined4 *)(*(int *)((int)local_6c + 0x10) + 0x3c));
    pvVar5 = FUN_00403d30((void *)(iVar4 + 0x2c));
    if ((pvVar5 == (void *)0x0) || ((*(byte *)((int)pvVar5 + 0x28) & 2) != 0)) goto LAB_004b07b1;
    FUN_004f26d0(&local_68,(undefined4 *)(*(int *)((int)local_6c + 0x10) + 0x3c));
    local_3c = (void *)0x90;
    local_38 = 0x98;
    if ((local_68 >> 0x18 < 0x90) || (0x97 < local_68 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) goto LAB_004b07b1;
    local_70 = (int *)FUN_004f5cd0(0x201);
    if (local_70 != (int *)0x0) {
      local_40 = (undefined4 *)FUN_00618b70(0x20);
      local_4._0_1_ = 3;
      if (local_40 == (undefined4 *)0x0) {
        pvVar5 = (void *)0x0;
      }
      else {
        pvVar5 = FUN_004f5b10(local_40,(undefined4 *)((int)this + 0x28),0);
      }
      local_4._0_1_ = 2;
      if (pvVar5 != (void *)0x0) {
        FUN_004f57b0(local_34,pvVar5);
        local_70[8] = *(int *)((int)local_6c + 4);
        (**(code **)(*local_70 + 0x24))();
        (**(code **)(*local_70 + 0x2c))(&local_6c);
        *param_1 = 1;
      }
      goto LAB_004b07b1;
    }
  }
  else {
LAB_004b07b1:
    if (local_70 != (int *)0x0) goto LAB_004b0b35;
  }
  if (((*(uint *)((int)pvVar3 + 0x38) & 0x4000000) != 0) &&
     ((*(uint *)((int)pvVar3 + 0x38) & 2) == 0)) {
    puVar10 = (undefined4 *)((int)pvVar3 + 0x28);
    iVar4 = *(int *)((int)local_6c + 0xc);
    local_40 = puVar10;
    FUN_004f26d0(&stack0xffffff7c,puVar10);
    pvVar5 = FUN_00403d30((void *)(iVar4 + 0x2c));
    if ((pvVar5 != (void *)0x0) && ((*(byte *)((int)pvVar5 + 0x30) & 1) != 0)) {
      local_5c = 0;
      thunk_FUN_005f50a0(local_34);
      pvVar3 = (void *)thunk_FUN_005f5060((int)pvVar3 + 0x48);
      pvVar5 = local_6c;
      for (; pvVar3 != (void *)0x0; pvVar3 = *(void **)((int)pvVar3 + 0x10)) {
        puVar6 = FUN_00403040(pvVar3,local_58);
        pvVar5 = local_6c;
        local_4._0_1_ = 4;
        pvVar7 = FUN_004f5940((void *)(*(int *)((int)local_6c + 0xc) + 0x58),puVar6);
        local_4._0_1_ = 2;
        FUN_00619730();
        puVar6 = FUN_00403040(pvVar3,local_54);
        local_4._0_1_ = 5;
        piVar8 = FUN_004f2e20(*(int *)((int)pvVar5 + 4),puVar6);
        local_4._0_1_ = 2;
        FUN_00619730();
        if ((((pvVar7 != (void *)0x0) && (piVar8 != (int *)0x0)) &&
            ((*(uint *)((int)pvVar7 + 0x24) & 0x800) == 0)) &&
           ((*(uint *)((int)pvVar7 + 0x24) & 0x600000) != 0)) {
          FUN_00536da0(local_2c,piVar8,1);
          local_4._0_1_ = 6;
          FUN_00513120((int)local_2c);
          while (local_10 != 0) {
            puVar6 = &local_50;
            pvVar7 = (void *)FUN_0052bed0((int)local_2c);
            puVar6 = FUN_004025b0(pvVar7,puVar6);
            local_4._0_1_ = 7;
            pvVar7 = FUN_004f5940((void *)(*(int *)((int)pvVar5 + 0xc) + 0x8c),puVar6);
            local_4._0_1_ = 6;
            FUN_00619730();
            if (pvVar7 != (void *)0x0) {
              if ((*(byte *)((int)pvVar7 + 0x30) & 2) == 0) {
                pvVar9 = (void *)FUN_00618b70(0x20);
                local_4._0_1_ = 0xc;
                local_3c = pvVar9;
                if (pvVar9 == (void *)0x0) {
                  pvVar7 = (void *)0x0;
                }
                else {
                  puVar10 = FUN_00403040(pvVar7,local_44);
                  uVar11 = uVar11 | 4;
                  local_4 = CONCAT31(local_4._1_3_,0xd);
                  local_60 = uVar11;
                  pvVar7 = FUN_004f5b10(pvVar9,puVar10,0);
                }
                local_4 = 0xc;
                if ((uVar11 & 4) != 0) {
                  uVar11 = uVar11 & 0xfffffffb;
                  FUN_00619730();
                }
                local_4._0_1_ = 6;
                if (pvVar7 != (void *)0x0) {
                  FUN_004f57b0(local_34,pvVar7);
                }
              }
              else {
                if (*(uint *)((int)pvVar7 + 0x28) == 0) {
                  pvVar9 = (void *)FUN_00618b70(0x20);
                  local_4._0_1_ = 8;
                  local_3c = pvVar9;
                  if (pvVar9 == (void *)0x0) {
                    pvVar7 = (void *)0x0;
                  }
                  else {
                    puVar10 = FUN_00403040(pvVar7,local_4c);
                    uVar11 = uVar11 | 1;
                    local_4 = CONCAT31(local_4._1_3_,9);
                    local_60 = uVar11;
                    pvVar7 = FUN_004f5b10(pvVar9,puVar10,0);
                  }
                  local_4 = 8;
                  if ((uVar11 & 1) != 0) {
                    uVar11 = uVar11 & 0xfffffffe;
LAB_004b0a0b:
                    FUN_00619730();
                  }
                }
                else {
                  bVar2 = FUN_00476e50(*(void **)((int)pvVar5 + 0x10),*(uint *)((int)pvVar7 + 0x28))
                  ;
                  if (CONCAT31(extraout_var,bVar2) != 0) goto LAB_004b0aa1;
                  pvVar9 = (void *)FUN_00618b70(0x20);
                  local_4._0_1_ = 10;
                  local_3c = pvVar9;
                  if (pvVar9 == (void *)0x0) {
                    pvVar7 = (void *)0x0;
                  }
                  else {
                    puVar10 = FUN_00403040(pvVar7,local_48);
                    uVar11 = uVar11 | 2;
                    local_4 = CONCAT31(local_4._1_3_,0xb);
                    local_60 = uVar11;
                    pvVar7 = FUN_004f5b10(pvVar9,puVar10,0);
                  }
                  local_4 = 10;
                  if ((uVar11 & 2) != 0) {
                    uVar11 = uVar11 & 0xfffffffd;
                    goto LAB_004b0a0b;
                  }
                }
                local_4._0_1_ = 6;
                if (pvVar7 != (void *)0x0) {
                  FUN_004f57b0(local_34,pvVar7);
                  local_5c = 1;
                }
              }
            }
LAB_004b0aa1:
            FUN_005130d0((int)local_2c);
          }
          local_4._0_1_ = 2;
          FUN_00536ea0(local_2c);
        }
        puVar10 = local_40;
      }
      iVar4 = thunk_FUN_005f5060((int)local_34);
      if (((iVar4 != 0) && (local_5c != 0)) &&
         (local_70 = (int *)FUN_004f5cd0(0x201), local_70 != (int *)0x0)) {
        local_70[8] = *(int *)((int)pvVar5 + 4);
        (**(code **)(*local_70 + 0x24))();
        (**(code **)(*local_70 + 0x2c))(puVar10);
      }
    }
  }
LAB_004b0b35:
  local_4._0_1_ = 1;
  FUN_004f4380(local_34);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return local_70;
}

