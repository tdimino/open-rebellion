
void __thiscall FUN_0046df90(void *this,undefined4 param_1,uint param_2)

{
  uint *puVar1;
  short sVar2;
  int iVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int *piVar6;
  uint *puVar7;
  undefined4 uVar8;
  char *pcVar9;
  bool bVar10;
  undefined1 *puVar11;
  uint local_a8;
  uint local_a4;
  undefined1 local_a0 [4];
  undefined1 local_9c [4];
  int *local_98;
  uint local_94;
  uint local_90;
  undefined4 local_8c [7];
  int local_70;
  undefined4 local_6c;
  undefined4 local_68;
  undefined4 local_64 [2];
  int local_5c;
  int local_48;
  undefined4 local_44;
  undefined4 local_40;
  undefined4 local_3c [7];
  int local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00632273;
  local_c = ExceptionList;
  local_98 = this;
  switch(param_2 & 0xffff) {
  case 100:
    ExceptionList = &local_c;
    FUN_0046ea10(this,param_2 >> 0x10);
    iVar3 = FUN_00609650(*(void **)((int)this + 0x14c),*(int *)((int)this + 0x148) + 0x98,0,0);
    if (iVar3 != 0) {
      puVar11 = local_9c;
      pvVar4 = (void *)FUN_0060a860(*(void **)((int)this + 0x178),iVar3);
      puVar5 = FUN_0042d170(pvVar4,puVar11);
      local_4 = 5;
      FUN_004f26d0((void *)((int)this + 0x17c),puVar5);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    SetFocus(*(HWND *)(*(int *)((int)this + 0x14c) + 0x18));
    break;
  case 200:
    ExceptionList = &local_c;
    (**(code **)(*(int *)this + 0x30))();
    break;
  case 0xc9:
    puVar1 = (uint *)((int)this + 0x17c);
    if (((*(uint *)((int)this + 0x17c) & 0xff000000) != 0) &&
       ((*(uint *)((int)this + 0x17c) & 0xffffff) != 0)) {
      ExceptionList = &local_c;
      piVar6 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x150) + 0x9c),puVar1);
      FUN_00504c40(local_8c,piVar6,3);
      bVar10 = false;
      local_4 = 0;
      FUN_00513120((int)local_8c);
      if (local_70 != 0) {
        local_6c = 0x90;
        local_68 = 0x98;
        if ((0x8f < *puVar1 >> 0x18) && (*puVar1 >> 0x18 < 0x98)) {
          bVar10 = true;
        }
        FUN_00619730();
        if (bVar10) {
          puVar7 = &local_a8;
          pvVar4 = (void *)FUN_0052bed0((int)local_8c);
          puVar7 = FUN_004025b0(pvVar4,puVar7);
          local_4._0_1_ = 1;
          FUN_004f26d0(puVar1,puVar7);
          local_4 = (uint)local_4._1_3_ << 8;
          FUN_00619730();
        }
        else {
          local_14 = 8;
          local_10 = 0x10;
          if ((*puVar1 >> 0x18 < 8) || (0xf < *puVar1 >> 0x18)) {
            bVar10 = false;
          }
          else {
            bVar10 = true;
          }
          FUN_00619730();
          if (bVar10) {
            FUN_00502db0(local_64,piVar6,3);
            local_4 = CONCAT31(local_4._1_3_,2);
            bVar10 = false;
            FUN_00513120((int)local_64);
            while ((local_48 != 0 && (!bVar10))) {
              uVar8 = FUN_0052bed0((int)local_64);
              FUN_00504c40(local_3c,uVar8,3);
              local_4._0_1_ = 3;
              FUN_00513120((int)local_3c);
              bVar10 = local_20 != 0;
              if (bVar10) {
                puVar7 = &local_90;
                pvVar4 = (void *)FUN_0052bed0((int)local_3c);
                puVar7 = FUN_004025b0(pvVar4,puVar7);
                local_4._0_1_ = 4;
                FUN_004f26d0(puVar1,puVar7);
                local_4._0_1_ = 3;
                FUN_00619730();
              }
              local_4 = CONCAT31(local_4._1_3_,2);
              FUN_00504d40(local_3c);
              FUN_005130d0((int)local_64);
            }
            local_4 = local_4 & 0xffffff00;
            FUN_00502eb0(local_64);
          }
        }
      }
      FUN_00429440(*(void **)((int)this + 0x150),puVar1);
      (**(code **)(*(int *)this + 0x30))();
      local_4 = 0xffffffff;
      FUN_00504d40(local_8c);
    }
    break;
  case 0xca:
    ExceptionList = &local_c;
    FUN_0060a790(local_64,1);
    local_4 = 6;
    FUN_00609410(*(void **)((int)this + 0x14c),local_64);
    if (local_5c != 0) {
      pvVar4 = (void *)FUN_0060a860(*(void **)((int)this + 0x178),*(int *)(local_5c + 0xc));
      puVar5 = FUN_0042d170(pvVar4,local_a0);
      puVar1 = (uint *)((int)this + 0x17c);
      local_4._0_1_ = 7;
      FUN_004f26d0(puVar1,puVar5);
      local_4._0_1_ = 6;
      FUN_00619730();
      sVar2 = (short)(param_2 >> 0x10);
      if ((sVar2 == 0x309) && (pvVar4 != (void *)0x0)) {
        piVar6 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x150) + 0x9c),puVar1);
        if (piVar6 != (int *)0x0) {
          local_44 = 0x90;
          local_40 = 0x98;
          if ((*puVar1 >> 0x18 < 0x90) || (0x97 < *puVar1 >> 0x18)) {
            bVar10 = false;
          }
          else {
            bVar10 = true;
          }
          FUN_00619730();
          if (bVar10) {
            FUN_00504c40(local_8c,piVar6,3);
            local_4._0_1_ = 8;
            FUN_00513120((int)local_8c);
            puVar7 = &local_94;
            pvVar4 = (void *)FUN_0052bed0((int)local_8c);
            puVar7 = FUN_004025b0(pvVar4,puVar7);
            local_4._0_1_ = 9;
            FUN_004f26d0(puVar1,puVar7);
            local_4._0_1_ = 8;
            FUN_00619730();
            local_4._0_1_ = 6;
            FUN_00504d40(local_8c);
          }
          else {
            local_1c = 8;
            local_18 = 0x10;
            if ((*puVar1 >> 0x18 < 8) || (0xf < *puVar1 >> 0x18)) {
              bVar10 = false;
            }
            else {
              bVar10 = true;
            }
            FUN_00619730();
            if (bVar10) {
              FUN_00502db0(local_3c,piVar6,3);
              local_4 = CONCAT31(local_4._1_3_,10);
              bVar10 = false;
              FUN_00513120((int)local_3c);
              while ((local_20 != 0 && (this = local_98, !bVar10))) {
                uVar8 = FUN_0052bed0((int)local_3c);
                FUN_00504c40(local_8c,uVar8,3);
                local_4._0_1_ = 0xb;
                FUN_00513120((int)local_8c);
                bVar10 = local_70 != 0;
                if (bVar10) {
                  puVar7 = &local_a4;
                  pvVar4 = (void *)FUN_0052bed0((int)local_8c);
                  puVar7 = FUN_004025b0(pvVar4,puVar7);
                  local_4._0_1_ = 0xc;
                  FUN_004f26d0(puVar1,puVar7);
                  local_4._0_1_ = 0xb;
                  FUN_00619730();
                }
                local_4 = CONCAT31(local_4._1_3_,10);
                FUN_00504d40(local_8c);
                FUN_005130d0((int)local_3c);
                this = local_98;
              }
              local_4._1_3_ = (uint3)(local_4 >> 8);
              local_4._0_1_ = 6;
              FUN_00502eb0(local_3c);
            }
          }
        }
        FUN_00429440(*(void **)((int)this + 0x150),puVar1);
        (**(code **)(*(int *)this + 0x30))();
      }
      else if (sVar2 == 0x29b) {
        pcVar9 = (char *)FUN_00583c40(local_5c + 0x14);
        FUN_00604f90(*(void **)((int)this + 0x148),pcVar9);
      }
    }
    local_4 = 0xffffffff;
    FUN_0060a810(local_64);
  }
  ExceptionList = local_c;
  return;
}

