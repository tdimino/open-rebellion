
void __thiscall FUN_0045b3e0(void *this,uint *param_1,uint param_2)

{
  bool bVar1;
  bool bVar2;
  bool bVar3;
  void *pvVar4;
  uint *puVar5;
  undefined4 *puVar6;
  uint uVar7;
  uint local_38 [11];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063098b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004025b0(*(void **)(param_2 + 0x1c),local_38);
  local_38[3] = 8;
  local_38[4] = 0x10;
  uVar7 = *param_1 >> 0x18;
  local_4 = 1;
  if ((uVar7 < 8) || (0xf < uVar7)) {
    local_38[1] = 0x28;
    local_38[2] = 0x30;
    if ((uVar7 < 0x28) || (0x2f < uVar7)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    FUN_00619730();
    if (bVar1) goto LAB_0045b477;
    bVar1 = false;
  }
  else {
LAB_0045b477:
    bVar1 = true;
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  local_38[9] = 0x1c;
  local_38[10] = 0x20;
  local_4 = CONCAT31(local_4._1_3_,2);
  if ((uVar7 < 0x1c) || (0x1f < uVar7)) {
    local_38[4] = 0x28;
    local_38[3] = 0x22;
    if ((uVar7 < 0x22) || (0x27 < uVar7)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      local_38[1] = 0x10;
      local_38[2] = 0x14;
      if ((uVar7 < 0x10) || (0x13 < uVar7)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (!bVar2) {
        local_38[5] = 0x20;
        local_38[6] = 0x22;
        if ((uVar7 < 0x20) || (0x21 < uVar7)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) {
          local_38[7] = 0x30;
          local_38[8] = 0x40;
          if ((uVar7 < 0x30) || (0x3f < uVar7)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (!bVar2) {
            bVar2 = false;
            goto LAB_0045b576;
          }
        }
      }
    }
  }
  bVar2 = true;
LAB_0045b576:
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  bVar3 = true;
  if (!bVar2) {
    bVar3 = bVar1;
  }
  if (bVar3) {
    pvVar4 = FUN_004f5940((void *)((int)this + 0x114),param_1);
    if (pvVar4 == (void *)0x0) {
      local_38[9] = 0x20;
      local_38[10] = 0x22;
      if ((*param_1 >> 0x18 < 0x20) || (0x21 < *param_1 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        pvVar4 = *(void **)(param_2 + 0x1c);
        puVar5 = FUN_004025b0(pvVar4,&param_2);
        local_4._0_1_ = 3;
        (**(code **)(*(int *)this + 100))(puVar5,pvVar4);
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_00619730();
      }
      else {
        puVar6 = (undefined4 *)FUN_00618b70(0x28);
        local_4._0_1_ = 4;
        if (puVar6 == (undefined4 *)0x0) {
          puVar6 = (undefined4 *)0x0;
        }
        else {
          puVar6 = FUN_004acd10(puVar6);
        }
        local_4 = (uint)local_4._1_3_ << 8;
        FUN_004f26d0(puVar6 + 6,param_1);
        FUN_004acba0((void *)((int)this + 0x114),puVar6,0);
        (**(code **)(*(int *)this + 100))(param_1,param_2);
      }
    }
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

