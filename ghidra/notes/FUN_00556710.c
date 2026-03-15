
uint FUN_00556710(int *param_1,undefined4 param_2)

{
  int *this;
  undefined4 uVar1;
  undefined1 uVar2;
  bool bVar3;
  undefined3 extraout_var;
  uint uVar4;
  uint *puVar5;
  uint *puVar6;
  int iVar7;
  int *piVar8;
  uint uVar9;
  int *piStack_60;
  int *piStack_5c;
  uint local_58;
  uint uStack_54;
  uint uStack_50;
  undefined4 auStack_4c [6];
  int iStack_34;
  undefined4 auStack_30 [9];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  this = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006496f0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  bVar3 = FUN_004f6b50(param_1,(int *)&param_1);
  uVar1 = param_2;
  uVar4 = CONCAT31(extraout_var,bVar3);
  if (((uVar4 != 0) && ((*(byte *)(this + 0x14) & 8) == 0)) &&
     ((*(byte *)(param_1 + 0x14) & 8) != 0)) {
    uVar4 = (**(code **)(*this + 0xb4))(0,param_2);
  }
  if (((uVar4 != 0) && ((this[0x14] & 0x1000U) == 0)) && ((this[0x14] & 8U) == 0)) {
    FUN_00520570(&uStack_54);
    uStack_4 = 0;
    uVar4 = (**(code **)(*param_1 + 0x54))(this,&uStack_54);
    if ((uVar4 != 0) && (uStack_50 != -1)) {
      uVar4 = (**(code **)(*this + 0xb0))(uVar1);
    }
    uStack_4 = 0xffffffff;
    FUN_00619730();
  }
  if (uVar4 != 0) {
    if (((this[0x14] & 0x1000U) == 0) && ((this[0x14] & 8U) == 0)) {
      FUN_00551060(auStack_30);
      uStack_50 = -1;
      uStack_54 = 1;
      piStack_5c = (int *)0x1;
      uVar4 = (uint)this[9] >> 6 & 3;
      uStack_4._0_1_ = 2;
      uStack_4._1_3_ = 0;
      uVar4 = FUN_00551100(auStack_30,uVar4,uVar4,0x3f0,&piStack_5c);
      if (uVar4 != 0) {
        puVar5 = FUN_004025b0(param_1,(uint *)&piStack_60);
        uStack_4._0_1_ = 3;
        uVar4 = FUN_00551270(auStack_30,puVar5);
        uStack_4._0_1_ = 2;
        FUN_00619730();
      }
      FUN_0054fa40(auStack_4c);
      uStack_4._0_1_ = 4;
      uVar2 = (undefined1)uStack_4;
      uStack_4._0_1_ = 4;
      if (uVar4 != 0) {
        if (uStack_50 == -1) {
          puVar5 = FUN_004025b0(param_1,(uint *)&piStack_60);
          uStack_4._0_1_ = 5;
          puVar6 = FUN_004025b0(this,(uint *)&piStack_5c);
          uStack_4._0_1_ = 6;
          uVar4 = FUN_00551c80(auStack_30,puVar6,puVar5,&uStack_54,auStack_4c);
          uStack_4._0_1_ = 5;
          FUN_00619730();
          uStack_4._0_1_ = 4;
          FUN_00619730();
          uVar2 = (undefined1)uStack_4;
        }
        uStack_4._0_1_ = uVar2;
        if (((uVar4 != 0) && (uStack_50 != -1)) && (iStack_34 != -1)) {
          uVar4 = (**(code **)(*this + 0xb8))(uVar1);
        }
      }
      uStack_4._0_1_ = 2;
      FUN_0054fa90(auStack_4c);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      FUN_00619730();
      uStack_4 = 0xffffffff;
      FUN_005510b0(auStack_30);
    }
    if (((uVar4 != 0) && (uVar9 = this[0x14], (uVar9 & 0x1000) == 0)) && ((uVar9 & 8) == 0)) {
      bVar3 = false;
      if ((uVar9 & 4) == 0) {
        FUN_004ece30(&piStack_5c);
        uStack_4 = 7;
        (**(code **)(*this + 0xc))(&piStack_5c);
        FUN_004ece30(&piStack_60);
        uStack_4 = CONCAT31(uStack_4._1_3_,8);
        iVar7 = FUN_004ece60((uint *)(this + 0xe));
        if ((iVar7 != 0) &&
           (piVar8 = (int *)FUN_00504dc0((uint *)(this + 0xe)), piVar8 != (int *)0x0)) {
          (**(code **)(*piVar8 + 0xc))(&piStack_60);
        }
        bVar3 = piStack_5c != piStack_60;
        uStack_4 = CONCAT31(uStack_4._1_3_,7);
        FUN_00619730();
        uStack_4 = 0xffffffff;
        FUN_00619730();
      }
      if (((*(byte *)(this + 0x14) & 0x10) != 0) || (bVar3)) {
        uVar4 = FUN_00555540(this,(int *)&piStack_5c);
        if (uVar4 != 0) {
          uStack_54 = 0x90;
          uStack_50 = 0x98;
          uStack_4 = 9;
          uVar9 = (**(code **)(*piStack_5c + 4))();
          if ((uVar9 < uStack_54) || (uStack_50 <= uVar9)) {
            bVar3 = false;
          }
          else {
            bVar3 = true;
          }
          uStack_4 = 0xffffffff;
          FUN_00619730();
          piVar8 = (int *)0x0;
          if (bVar3) {
            piVar8 = piStack_5c;
          }
          if ((((uVar4 != 0) && (piVar8 != (int *)0x0)) && ((*(byte *)(piVar8 + 0x22) & 0x20) != 0))
             && (uVar4 = FUN_00509890(piVar8,&local_58), uVar4 != 0)) {
            uStack_54 = 8;
            uStack_50 = 0x10;
            uStack_4 = 10;
            uVar9 = (**(code **)(*this + 4))();
            if ((uVar9 < uStack_54) || (uStack_50 <= uVar9)) {
              bVar3 = false;
            }
            else {
              bVar3 = true;
            }
            uStack_4 = 0xffffffff;
            FUN_00619730();
            if (!bVar3) {
              uStack_54 = 0x14;
              uStack_50 = 0x1c;
              uStack_4 = 0xb;
              uVar9 = (**(code **)(*this + 4))();
              if ((uVar9 < uStack_54) || (uStack_50 <= uVar9)) {
                bVar3 = false;
              }
              else {
                bVar3 = true;
              }
              uStack_4 = 0xffffffff;
              FUN_00619730();
              if ((!bVar3) && (local_58 != ((uint)this[9] >> 6 & 3))) {
                uVar4 = (**(code **)(*this + 0xb8))(param_2);
              }
            }
          }
        }
      }
    }
  }
  ExceptionList = pvStack_c;
  return uVar4;
}

