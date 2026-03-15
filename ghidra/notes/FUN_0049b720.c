
void __thiscall FUN_0049b720(void *this,uint param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  void *this_00;
  int *piVar4;
  void *pvVar5;
  int iVar6;
  void *pvVar7;
  uint *puVar8;
  uint local_38;
  uint local_34;
  void *local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  uVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636e48;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 0x5c) = *(undefined4 *)(param_1 + 0x4c);
  pvVar3 = (void *)thunk_FUN_005f5060(param_1 + 0x6c);
  this_00 = (void *)thunk_FUN_005f5060(uVar2 + 0x74);
  while (pvVar3 != (void *)0x0) {
    FUN_00403040(pvVar3,&param_1);
    local_4 = 0;
    piVar4 = FUN_004f2d10(*(int *)((int)this + 0x20),&param_1);
    if (piVar4 != (int *)0x0) {
      if ((piVar4[0x14] & 8U) == 0) {
        if ((piVar4[0x14] & 0x20U) != 0) {
          local_2c = 8;
          local_28 = 0x10;
          if ((param_1 >> 0x18 < 8) || (0xf < param_1 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            iVar6 = FUN_004ece60((uint *)((int)this + 0x48));
            if ((iVar6 == 0) && (*(int *)((int)this + 0x20) == 1)) {
              puVar8 = FUN_004025b0((void *)piVar4[7],&local_38);
              local_4._0_1_ = 2;
              FUN_004f26d0((uint *)((int)this + 0x48),puVar8);
              local_4 = (uint)local_4._1_3_ << 8;
              FUN_00619730();
            }
            pvVar7 = (void *)((int)this + 0x60);
            pvVar5 = FUN_004f5940(pvVar7,&param_1);
            if (pvVar5 == (void *)0x0) {
              local_30 = (void *)FUN_00618b70(0x1c);
              local_4 = CONCAT31(local_4._1_3_,3);
              if (local_30 == (void *)0x0) {
LAB_0049b9d9:
                pvVar5 = (void *)0x0;
              }
              else {
                pvVar5 = FUN_004ecc70(local_30,&param_1);
              }
LAB_0049b9db:
              local_4 = local_4 & 0xffffff00;
              FUN_004f57b0(pvVar7,pvVar5);
            }
          }
          else {
            local_24 = 0x1c;
            local_20 = 0x20;
            if ((param_1 >> 0x18 < 0x1c) || (0x1f < param_1 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (bVar1) {
              iVar6 = FUN_004ece60((uint *)((int)this + 0x48));
              if ((iVar6 == 0) && (*(int *)((int)this + 0x20) == 1)) {
                puVar8 = FUN_004025b0((void *)piVar4[7],&local_34);
                local_4._0_1_ = 4;
                FUN_004f26d0((uint *)((int)this + 0x48),puVar8);
                local_4 = (uint)local_4._1_3_ << 8;
                FUN_00619730();
              }
              pvVar7 = (void *)((int)this + 0x68);
              pvVar5 = FUN_004f5940(pvVar7,&param_1);
              if (pvVar5 == (void *)0x0) {
                local_30 = (void *)FUN_00618b70(0x1c);
                local_4 = CONCAT31(local_4._1_3_,5);
                if (local_30 == (void *)0x0) goto LAB_0049b9d9;
                pvVar5 = FUN_004ecc70(local_30,&param_1);
                goto LAB_0049b9db;
              }
            }
            else {
              local_1c = 0x30;
              local_18 = 0x3c;
              if ((param_1 >> 0x18 < 0x30) || (0x3b < param_1 >> 0x18)) {
                bVar1 = false;
              }
              else {
                bVar1 = true;
              }
              FUN_00619730();
              if (bVar1) {
                pvVar7 = (void *)((int)this + 0x70);
                pvVar5 = FUN_004f5940(pvVar7,&param_1);
                if (pvVar5 == (void *)0x0) {
                  local_30 = (void *)FUN_00618b70(0x1c);
                  local_4 = CONCAT31(local_4._1_3_,6);
                  if (local_30 == (void *)0x0) goto LAB_0049b9d9;
                  pvVar5 = FUN_004ecc70(local_30,&param_1);
                  goto LAB_0049b9db;
                }
              }
            }
          }
        }
      }
      else {
        pvVar7 = (void *)((int)this + 0x78);
        pvVar5 = FUN_004f5940(pvVar7,&param_1);
        if (pvVar5 == (void *)0x0) {
          local_30 = (void *)FUN_00618b70(0x1c);
          local_4 = CONCAT31(local_4._1_3_,1);
          if (local_30 == (void *)0x0) goto LAB_0049b9d9;
          pvVar5 = FUN_004eccd0(local_30,(int)pvVar3);
          goto LAB_0049b9db;
        }
      }
      local_14 = 8;
      local_10 = 0x10;
      if (((param_1 >> 0x18 < 8) || (0xf < param_1 >> 0x18)) ||
         ((~(byte)((uint)piVar4[0x14] >> 3) & 1) == 0)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if ((bVar1) && (*(int *)((int)this + 0x20) == 1)) {
        iVar6 = FUN_004ece60((uint *)((int)this + 0x4c));
        if (iVar6 == 0) {
          FUN_004f26d0((uint *)((int)this + 0x4c),&param_1);
        }
      }
    }
    pvVar3 = *(void **)((int)pvVar3 + 0x10);
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  do {
    if (this_00 == (void *)0x0) {
      ExceptionList = local_c;
      return;
    }
    FUN_00403040(this_00,&param_1);
    local_4 = 7;
    piVar4 = FUN_004f2d10(*(int *)((int)this + 0x20),&param_1);
    if (piVar4 != (int *)0x0) {
      if ((piVar4[0x14] & 8U) == 0) {
        if ((piVar4[0x14] & 0x20U) != 0) {
          local_14 = 8;
          local_10 = 0x10;
          if ((param_1 >> 0x18 < 8) || (0xf < param_1 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          if (bVar1) {
            iVar6 = FUN_004ece60((uint *)((int)this + 0x48));
            if ((iVar6 == 0) && (*(int *)((int)this + 0x20) == 2)) {
              puVar8 = FUN_004025b0((void *)piVar4[7],&local_34);
              local_4._0_1_ = 9;
              FUN_004f26d0((uint *)((int)this + 0x48),puVar8);
              local_4 = CONCAT31(local_4._1_3_,7);
              FUN_00619730();
            }
            pvVar3 = (void *)((int)this + 0x60);
            pvVar7 = FUN_004f5940(pvVar3,&param_1);
            if (pvVar7 == (void *)0x0) {
              local_30 = (void *)FUN_00618b70(0x1c);
              local_4 = CONCAT31(local_4._1_3_,10);
              if (local_30 == (void *)0x0) {
LAB_0049bcf0:
                pvVar7 = (void *)0x0;
              }
              else {
                pvVar7 = FUN_004ecc70(local_30,&param_1);
              }
LAB_0049bcf2:
              local_4 = CONCAT31(local_4._1_3_,7);
              FUN_004f57b0(pvVar3,pvVar7);
            }
          }
          else {
            local_1c = 0x1c;
            local_18 = 0x20;
            if ((param_1 >> 0x18 < 0x1c) || (0x1f < param_1 >> 0x18)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            FUN_00619730();
            if (bVar1) {
              iVar6 = FUN_004ece60((uint *)((int)this + 0x48));
              if ((iVar6 == 0) && (*(int *)((int)this + 0x20) == 2)) {
                puVar8 = FUN_004025b0((void *)piVar4[7],&local_38);
                local_4._0_1_ = 0xb;
                FUN_004f26d0((uint *)((int)this + 0x48),puVar8);
                local_4 = CONCAT31(local_4._1_3_,7);
                FUN_00619730();
              }
              pvVar3 = (void *)((int)this + 0x68);
              pvVar7 = FUN_004f5940(pvVar3,&param_1);
              if (pvVar7 == (void *)0x0) {
                local_30 = (void *)FUN_00618b70(0x1c);
                local_4 = CONCAT31(local_4._1_3_,0xc);
                if (local_30 == (void *)0x0) goto LAB_0049bcf0;
                pvVar7 = FUN_004ecc70(local_30,&param_1);
                goto LAB_0049bcf2;
              }
            }
            else {
              local_24 = 0x30;
              local_20 = 0x3c;
              if ((param_1 >> 0x18 < 0x30) || (0x3b < param_1 >> 0x18)) {
                bVar1 = false;
              }
              else {
                bVar1 = true;
              }
              FUN_00619730();
              if (bVar1) {
                pvVar3 = (void *)((int)this + 0x70);
                pvVar7 = FUN_004f5940(pvVar3,&param_1);
                if (pvVar7 == (void *)0x0) {
                  local_30 = (void *)FUN_00618b70(0x1c);
                  local_4 = CONCAT31(local_4._1_3_,0xd);
                  if (local_30 == (void *)0x0) goto LAB_0049bcf0;
                  pvVar7 = FUN_004ecc70(local_30,&param_1);
                  goto LAB_0049bcf2;
                }
              }
            }
          }
        }
      }
      else {
        pvVar3 = (void *)((int)this + 0x80);
        pvVar7 = FUN_004f5940(pvVar3,&param_1);
        if (pvVar7 == (void *)0x0) {
          local_30 = (void *)FUN_00618b70(0x1c);
          local_4 = CONCAT31(local_4._1_3_,8);
          if (local_30 == (void *)0x0) goto LAB_0049bcf0;
          pvVar7 = FUN_004eccd0(local_30,(int)this_00);
          goto LAB_0049bcf2;
        }
      }
      local_2c = 8;
      local_28 = 0x10;
      if (((param_1 >> 0x18 < 8) || (0xf < param_1 >> 0x18)) ||
         ((~(byte)((uint)piVar4[0x14] >> 3) & 1) == 0)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if ((bVar1) && (*(int *)((int)this + 0x20) == 2)) {
        iVar6 = FUN_004ece60((uint *)((int)this + 0x4c));
        if (iVar6 == 0) {
          FUN_004f26d0((uint *)((int)this + 0x4c),&param_1);
        }
      }
    }
    local_4 = 0xffffffff;
    this_00 = *(void **)((int)this_00 + 0x10);
    FUN_00619730();
  } while( true );
}

