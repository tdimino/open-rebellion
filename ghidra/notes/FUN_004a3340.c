
void __fastcall FUN_004a3340(int *param_1)

{
  uint uVar1;
  uint uVar2;
  int *piVar3;
  undefined4 *puVar4;
  void *pvVar5;
  uint *puVar6;
  int iStack_4c;
  undefined4 local_48;
  uint uStack_44;
  uint uStack_40;
  uint uStack_3c;
  undefined4 *puStack_38;
  int iStack_34;
  undefined4 *puStack_30;
  undefined4 local_2c [7];
  int iStack_10;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637a88;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar3 = FUN_004a25c0(param_1);
  if (piVar3 == (int *)0x0) {
    ExceptionList = local_c;
    return;
  }
  local_48 = FUN_006037f0(7);
  FUN_004ffe70(local_2c,piVar3,3);
  local_4 = 0;
  iStack_4c = (**(code **)(*(int *)param_1[0x62] + 8))();
  iStack_34 = param_1[0x5b];
  (**(code **)(param_1[0x45] + 4))();
  puStack_38 = (undefined4 *)FUN_00618b70(0x28);
  local_4._0_1_ = 1;
  if (puStack_38 == (undefined4 *)0x0) {
    puVar4 = (undefined4 *)0x0;
  }
  else {
    puVar4 = FUN_004acd10(puStack_38);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_004f26d0(puVar4 + 6,param_1 + 0x51);
  FUN_004ace40(puVar4,param_1[9]);
  FUN_004acba0(param_1 + 0x45,puVar4,0);
  FUN_00513120((int)local_2c);
  if (iStack_10 == 0) {
    param_1[0x5b] = (uint)piVar3[9] >> 6 & 3;
  }
  else {
    param_1[0x5b] = 2 - (uint)(param_1[0x5c] != 1);
    do {
      pvVar5 = (void *)FUN_0052bed0((int)local_2c);
      if ((*(uint *)((int)pvVar5 + 0x24) >> 6 & 3) == param_1[0x5c]) {
        param_1[0x5b] = param_1[0x5c];
      }
      if (iStack_4c == 0) {
        FUN_004a37c0(param_1,pvVar5,&iStack_4c);
        FUN_005130d0((int)local_2c);
      }
      else {
        puVar6 = FUN_004025b0(pvVar5,&uStack_44);
        uVar1 = *(uint *)(iStack_4c + 0x6c);
        uVar2 = *puVar6;
        FUN_00619730();
        if (uVar2 < uVar1) {
          FUN_004a37c0(param_1,pvVar5,&iStack_4c);
          FUN_005130d0((int)local_2c);
        }
        else {
          puVar6 = FUN_004025b0(pvVar5,&uStack_40);
          uVar1 = *puVar6;
          uVar2 = *(uint *)(iStack_4c + 0x6c);
          FUN_00619730();
          if (uVar2 == uVar1) {
            puStack_30 = (undefined4 *)FUN_00618b70(0x28);
            local_4._0_1_ = 2;
            if (puStack_30 == (undefined4 *)0x0) {
              puVar4 = (undefined4 *)0x0;
            }
            else {
              puVar4 = FUN_004acd10(puStack_30);
            }
            local_4._0_1_ = 0;
            puVar6 = FUN_004025b0(pvVar5,&uStack_3c);
            local_4._0_1_ = 3;
            FUN_004f26d0(puVar4 + 6,puVar6);
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_00619730();
            FUN_004ace40(puVar4,param_1[9]);
            FUN_004acba0(param_1 + 0x45,puVar4,0);
            FUN_004a3d40(param_1,(int)pvVar5,&iStack_4c);
            FUN_005130d0((int)local_2c);
          }
          else {
            puVar6 = FUN_004025b0(pvVar5,(uint *)&puStack_38);
            uVar1 = *puVar6;
            uVar2 = *(uint *)(iStack_4c + 0x6c);
            FUN_00619730();
            if (uVar2 < uVar1) {
              FUN_004a4ac0(param_1,&iStack_4c);
            }
          }
        }
      }
    } while (iStack_10 != 0);
  }
  while (iStack_4c != 0) {
    FUN_004a4ac0(param_1,&iStack_4c);
  }
  if (param_1[0x5b] == iStack_34) goto LAB_004a3780;
  puVar4 = (undefined4 *)param_1[0x5e];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  puVar4 = (undefined4 *)param_1[0x5f];
  if (puVar4 != (undefined4 *)0x0) {
    FUN_005fbfa0(puVar4);
    FUN_00618b60((undefined *)puVar4);
  }
  if (param_1[0x5b] == 1) {
    puStack_30 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 4;
    if (puStack_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(puStack_30,local_48,0x283b,10);
    }
    local_4._0_1_ = 0;
    param_1[0x5e] = (int)pvVar5;
    puStack_30 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,5);
    if (puStack_30 == (undefined4 *)0x0) goto LAB_004a3751;
    pvVar5 = FUN_005fbd20(puStack_30,local_48,0x27d8,10);
  }
  else if (param_1[0x5b] == 2) {
    puStack_30 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 6;
    if (puStack_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(puStack_30,local_48,0x27d9,10);
    }
    local_4._0_1_ = 0;
    param_1[0x5e] = (int)pvVar5;
    puStack_30 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,7);
    if (puStack_30 == (undefined4 *)0x0) goto LAB_004a3751;
    pvVar5 = FUN_005fbd20(puStack_30,local_48,0x283e,10);
  }
  else {
    puStack_30 = (undefined4 *)FUN_00618b70(0x28);
    local_4._0_1_ = 8;
    if (puStack_30 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(puStack_30,local_48,0x283f,10);
    }
    local_4._0_1_ = 0;
    param_1[0x5e] = (int)pvVar5;
    puStack_30 = (undefined4 *)FUN_00618b70(0x28);
    local_4 = CONCAT31(local_4._1_3_,9);
    if (puStack_30 == (undefined4 *)0x0) {
LAB_004a3751:
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005fbd20(puStack_30,local_48,0x2840,10);
    }
  }
  local_4 = local_4 & 0xffffff00;
  param_1[0x5f] = (int)pvVar5;
  if ((void *)param_1[0x49] != (void *)0x0) {
    FUN_00601c90((void *)param_1[0x49],0,(HWND)0x0);
  }
  InvalidateRect((HWND)param_1[6],(RECT *)(param_1 + 0x4a),0);
LAB_004a3780:
  if ((void *)param_1[0x58] != (void *)0x0) {
    FUN_0060a280((void *)param_1[0x58]);
  }
  FUN_004a5c00(param_1);
  local_4 = 0xffffffff;
  FUN_004fff70(local_2c);
  ExceptionList = local_c;
  return;
}

