
undefined4 __fastcall FUN_0047a050(void *param_1)

{
  uint *puVar1;
  bool bVar2;
  bool bVar3;
  void *pvVar4;
  int iVar5;
  void *pvVar6;
  uint uVar7;
  undefined4 *puVar8;
  uint local_2c;
  undefined4 local_28;
  uint local_24;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00633480;
  local_c = ExceptionList;
  bVar3 = false;
  local_20 = 0;
  ExceptionList = &local_c;
  FUN_004ece30(&local_24);
  local_4 = 0;
  FUN_004ece30(&local_28);
  local_4._0_1_ = 1;
  FUN_004ece30(&local_2c);
  puVar1 = (uint *)((int)param_1 + 0x2c);
  local_4 = CONCAT31(local_4._1_3_,2);
  *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) & 0xffffffe2;
  local_1c = 0x30;
  local_18 = 0x40;
  if ((*puVar1 >> 0x18 < 0x30) || (0x3f < *puVar1 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    pvVar4 = FUN_004f5940(*(void **)((int)param_1 + 0x40),puVar1);
    if (pvVar4 != (void *)0x0) {
      if (*(int *)((int)pvVar4 + 0x28) == *(int *)((int)param_1 + 0x18)) {
        if ((*(byte *)((int)pvVar4 + 0x30) & 0x50) == 0) {
          *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) | 1;
          if (*(int *)((int)pvVar4 + 0x38) == *(int *)((int)param_1 + 0x28)) {
            uVar7 = *(uint *)((int)param_1 + 0x20);
            *(uint *)((int)param_1 + 0x20) = uVar7 | 8;
            if ((*(uint *)((int)pvVar4 + 0x30) & 0x7000000) != 0) {
              *(uint *)((int)param_1 + 0x20) = uVar7 | 0xc;
            }
          }
          goto LAB_0047a146;
        }
        *(uint *)((int)pvVar4 + 0x30) = *(uint *)((int)pvVar4 + 0x30) | 0x800000;
        *(undefined4 *)((int)pvVar4 + 0x24) = 0;
        *(undefined4 *)((int)pvVar4 + 0x2c) = 0;
        *(undefined4 *)((int)pvVar4 + 0x28) = 0;
      }
      FUN_004ece80(puVar1);
    }
  }
LAB_0047a146:
  if ((*(byte *)((int)param_1 + 0x20) & 1) == 0) {
    iVar5 = FUN_005f5500((void *)(*(int *)((int)param_1 + 0x44) + 0xd8),
                         *(uint *)((int)param_1 + 0x54));
    if (iVar5 != 0) {
      FUN_004f26d0(&local_24,(undefined4 *)(iVar5 + 0x34));
      pvVar6 = FUN_00403460(*(void **)((int)param_1 + 0x40),0x800002,*(uint *)((int)param_1 + 0x50),
                            0x474000f1,0,6,0x32,10000,(void *)0x1);
      pvVar4 = (void *)((int)param_1 + 0x30);
      FUN_00435790(pvVar4,(int)pvVar6);
      uVar7 = *(uint *)((int)param_1 + 0x48) >> 0x18;
      local_1c = 0x90;
      local_18 = 0x98;
      if ((0x8f < uVar7) && (uVar7 < 0x98)) {
        bVar3 = true;
      }
      FUN_00619730();
      if (bVar3) {
        FUN_004f26d0(&local_28,(undefined4 *)((int)param_1 + 0x48));
      }
      else {
        pvVar6 = FUN_004f5940((void *)(*(int *)((int)param_1 + 0x44) + 0x44),&local_24);
        if (pvVar6 != (void *)0x0) {
          puVar8 = FUN_00432120(pvVar6,&local_1c,0);
          local_4._0_1_ = 3;
          FUN_004f26d0(&local_28,puVar8);
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_00619730();
        }
      }
      puVar8 = FUN_00403750(*(void **)((int)param_1 + 0x40),&local_28,0x800002,
                            *(uint *)((int)param_1 + 0x50),0x474000f1,0,2);
      FUN_00435790(pvVar4,(int)puVar8);
      puVar8 = FUN_004357b0(pvVar4,&local_1c);
      local_4._0_1_ = 4;
      FUN_004f26d0(&local_2c,puVar8);
      while( true ) {
        local_4 = CONCAT31(local_4._1_3_,2);
        FUN_00619730();
        local_14 = 0x30;
        local_10 = 0x40;
        if (((local_2c >> 0x18 < 0x30) || (0x3f < local_2c >> 0x18)) ||
           ((~*(byte *)((int)param_1 + 0x20) & 1) == 0)) {
          bVar3 = false;
        }
        else {
          bVar3 = true;
        }
        FUN_00619730();
        if (!bVar3) break;
        pvVar6 = FUN_004f5940(*(void **)((int)param_1 + 0x40),&local_2c);
        if (pvVar6 != (void *)0x0) {
          iVar5 = FUN_004bc180(param_1,&local_2c);
          if (iVar5 != 0) {
            FUN_004f26d0((void *)((int)param_1 + 0x2c),&local_2c);
            *(uint *)((int)pvVar6 + 0x30) = *(uint *)((int)pvVar6 + 0x30) & 0xff7fffff;
            *(undefined4 *)((int)pvVar6 + 0x24) = 0;
            *(undefined4 *)((int)pvVar6 + 0x28) = *(undefined4 *)((int)param_1 + 0x18);
            *(undefined4 *)((int)pvVar6 + 0x2c) = *(undefined4 *)((int)param_1 + 0x4c);
            FUN_004021b0((int)pvVar6);
            *(uint *)((int)param_1 + 0x20) = *(uint *)((int)param_1 + 0x20) | 1;
          }
        }
        puVar8 = FUN_004357b0(pvVar4,&local_1c);
        local_4._0_1_ = 5;
        FUN_004f26d0(&local_2c,puVar8);
      }
      FUN_005f58b0(pvVar4);
    }
    if ((*(byte *)((int)param_1 + 0x20) & 1) == 0) goto LAB_0047a356;
  }
  local_20 = 1;
LAB_0047a356:
  local_4._0_1_ = 1;
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return local_20;
}

