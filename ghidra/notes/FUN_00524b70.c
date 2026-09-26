
undefined4 __thiscall FUN_00524b70(void *this,undefined4 param_1,int param_2,void *param_3)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  uint *puVar3;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  uint uVar4;
  undefined4 uVar5;
  undefined3 extraout_var_05;
  undefined3 extraout_var_06;
  uint uVar6;
  uint local_44;
  uint local_40;
  undefined4 local_3c [11];
  int *local_10;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00643cf8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_2 != 0xb) {
    ExceptionList = &local_c;
    *(int *)(*(int *)((int)this + 0x54) + 0x1c) = param_2;
  }
  iVar2 = FUN_00522a90(this,&local_40);
  uVar6 = (uint)(iVar2 != 0);
  switch(param_1) {
  case 0:
  case 1:
  case 2:
  case 3:
  case 4:
  case 5:
  case 6:
  case 7:
  case 9:
  case 10:
  case 0xb:
    break;
  case 8:
    iVar2 = FUN_00522480(this,param_3);
    if ((iVar2 == 0) || (uVar6 == 0)) {
      bVar1 = false;
    }
    else {
      bVar1 = true;
    }
    uVar6 = FUN_0053fa60(0x38b,0,this,(int *)(*(int *)((int)this + 0x54) + 0x20),param_3);
    if ((uVar6 == 0) || (!bVar1)) goto switchD_00524bc2_default;
    uVar6 = 1;
    break;
  default:
switchD_00524bc2_default:
    uVar6 = 0;
  }
  switch(param_2) {
  case 0:
  case 1:
  case 3:
  case 7:
  case 9:
  case 10:
    goto switchD_00524c14_caseD_0;
  case 2:
    bVar1 = FUN_00522870(this,param_3);
    uVar4 = CONCAT31(extraout_var,bVar1);
    break;
  case 4:
    iVar2 = FUN_00522480(this,param_3);
    if ((iVar2 == 0) || (uVar6 == 0)) {
      uVar6 = 0;
    }
    else {
      uVar6 = 1;
    }
    bVar1 = FUN_00520e40((int)this);
    if (CONCAT31(extraout_var_00,bVar1) == 0) {
      iVar2 = thunk_FUN_00506e60();
      uVar6 = (uint)(iVar2 != 0);
      if (iVar2 != 0) {
        FUN_00525bb0(local_3c,this);
        local_4 = 0;
        FUN_00525930((int)local_3c);
        if (local_10 != (int *)0x0) {
          do {
            uVar4 = FUN_00556430(local_10,(uint *)((int)this + 0x6c),(uint *)((int)this + 0x78),
                                 param_3);
            if ((uVar4 == 0) || (uVar6 == 0)) {
              bVar1 = false;
            }
            else {
              bVar1 = true;
            }
            puVar3 = FUN_004025b0(local_10,&local_44);
            local_4._0_1_ = 1;
            iVar2 = FUN_00556390(puVar3,(uint *)((int)this + 0x74),param_3);
            if ((iVar2 == 0) || (!bVar1)) {
              uVar6 = 0;
            }
            else {
              uVar6 = 1;
            }
            local_4 = (uint)local_4._1_3_ << 8;
            FUN_00619730();
            if ((uVar6 != 0) &&
               (bVar1 = FUN_00520bb0((int)this), CONCAT31(extraout_var_01,bVar1) != 0)) {
              uVar6 = (local_10[0x14] & 0xffU) >> 5 & 1;
            }
            FUN_005258f0((int)local_3c);
          } while (local_10 != (int *)0x0);
        }
        local_4 = 0xffffffff;
        FUN_00525c50(local_3c);
      }
    }
    bVar1 = FUN_00522280(this,param_3);
    uVar4 = CONCAT31(extraout_var_02,bVar1);
    break;
  case 5:
    bVar1 = FUN_00520e40((int)this);
    if ((CONCAT31(extraout_var_03,bVar1) != 0) ||
       (bVar1 = FUN_00520bb0((int)this), CONCAT31(extraout_var_04,bVar1) == 0))
    goto switchD_00524c14_caseD_0;
  case 0xb:
    uVar4 = FUN_00522480(this,param_3);
    break;
  case 6:
    bVar1 = FUN_00520e40((int)this);
    if ((CONCAT31(extraout_var_05,bVar1) == 0) &&
       (bVar1 = FUN_00520bb0((int)this), CONCAT31(extraout_var_06,bVar1) != 0)) {
      FUN_00525bb0(local_3c,this);
      local_4 = 2;
      FUN_00525930((int)local_3c);
      while (local_10 != (int *)0x0) {
        iVar2 = FUN_004f7640(local_10,0,param_3);
        if ((iVar2 == 0) || (uVar6 == 0)) {
          uVar6 = 0;
        }
        else {
          uVar6 = 1;
        }
        FUN_005258f0((int)local_3c);
      }
      iVar2 = FUN_00522480(this,param_3);
      if ((iVar2 == 0) || (uVar6 == 0)) {
        uVar6 = 0;
      }
      else {
        uVar6 = 1;
      }
      local_4 = 0xffffffff;
      FUN_00525c50(local_3c);
    }
    goto switchD_00524c14_caseD_0;
  case 8:
    iVar2 = FUN_00521880(this,0,param_3);
    if ((iVar2 == 0) || (uVar6 == 0)) {
      uVar6 = 0;
    }
    else {
      uVar6 = 1;
    }
    uVar4 = FUN_0053fa60(0x38b,1,this,(int *)(*(int *)((int)this + 0x54) + 0x20),param_3);
    break;
  default:
    goto switchD_00524c14_default;
  }
  if ((uVar4 == 0) || (uVar6 == 0)) {
switchD_00524c14_default:
    uVar6 = 0;
  }
  else {
    uVar6 = 1;
  }
switchD_00524c14_caseD_0:
  uVar4 = FUN_0053f950(0x30d,this,param_1,param_2,param_3);
  if ((uVar4 == 0) || (uVar6 == 0)) {
    uVar5 = 0;
  }
  else {
    uVar5 = 1;
  }
  ExceptionList = local_c;
  return uVar5;
}

