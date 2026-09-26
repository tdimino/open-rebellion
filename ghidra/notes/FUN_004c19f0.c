
void __thiscall FUN_004c19f0(void *this,int *param_1,int *param_2)

{
  bool bVar1;
  int *piVar2;
  uint *puVar3;
  int iVar4;
  int *piVar5;
  void *this_00;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar5 = param_2;
  puStack_8 = &LAB_0063a4e0;
  local_c = ExceptionList;
  local_4 = 0;
  if ((*param_2 == 1) && (ExceptionList = &local_c, param_2[1] == 0x16)) {
LAB_004c1a66:
    bVar1 = true;
  }
  else {
    if ((*param_2 == 1) && (param_2[1] == 0x12)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    ExceptionList = &local_c;
    FUN_00619730();
    if (bVar1) goto LAB_004c1a66;
    bVar1 = false;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    ExceptionList = local_c;
    return;
  }
  if ((*piVar5 == 1) && (piVar5[1] == 0x26)) {
    bVar1 = true;
  }
  else {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    ExceptionList = local_c;
    return;
  }
  if ((*piVar5 == 0x40) && (piVar5[1] == 0x92)) {
    bVar1 = true;
  }
  else {
    bVar1 = false;
  }
  FUN_00619730();
  if (bVar1) {
    puVar3 = (uint *)(**(code **)(*param_1 + 0x30))(&param_1);
    local_4 = 1;
    iVar4 = FUN_004ece60(puVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (iVar4 != 0) {
      *(int *)(*(int *)((int)this + 0x168) + 0xc) = DAT_006b28cc + 10;
      ExceptionList = local_c;
      return;
    }
    goto LAB_004c21a9;
  }
  if ((*piVar5 == 0x40) && (piVar5[1] == 0x91)) {
    bVar1 = true;
  }
  else {
    bVar1 = false;
  }
  FUN_00619730();
  piVar2 = param_1;
  if (!bVar1) {
    if ((*piVar5 == 1) && (piVar5[1] == 0x18)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    FUN_00619730();
    if (bVar1) {
      *(int *)(*(int *)((int)this + 0x168) + 0x14) = DAT_006b28cc + 10;
      ExceptionList = local_c;
      return;
    }
    if ((*piVar5 == 1) && (piVar5[1] == 0x22)) {
      bVar1 = true;
    }
    else {
      bVar1 = false;
    }
    FUN_00619730();
    piVar2 = param_1;
    if (!bVar1) {
      local_4 = 6;
      if ((*piVar5 == 1) && (piVar5[1] == 0x27)) {
LAB_004c1da5:
        bVar1 = true;
      }
      else {
        if ((*piVar5 == 1) && (piVar5[1] == 0x25)) {
          bVar1 = true;
        }
        else {
          bVar1 = false;
        }
        FUN_00619730();
        if (bVar1) goto LAB_004c1da5;
        bVar1 = false;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      piVar2 = param_1;
      if (bVar1) {
        puVar3 = (uint *)(**(code **)(*param_1 + 0x30))(&param_2);
        local_4 = 7;
        iVar4 = FUN_004ece60(puVar3);
        if (iVar4 != 0) {
          puVar3 = (uint *)(**(code **)(*piVar2 + 0x30))(&param_1);
          local_4._0_1_ = 8;
          if ((*puVar3 >> 0x18 < 8) || (0xf < *puVar3 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          local_4 = CONCAT31(local_4._1_3_,7);
          FUN_00619730();
          if (bVar1) {
            bVar1 = true;
            goto LAB_004c1e38;
          }
        }
        bVar1 = false;
LAB_004c1e38:
        local_4 = 0xffffffff;
        FUN_00619730();
        if (bVar1) {
          *(int *)(*(int *)((int)this + 0x168) + 0x1c) = DAT_006b28cc + 10;
          ExceptionList = local_c;
          return;
        }
        *(int *)(*(int *)((int)this + 0x168) + 0x60) = DAT_006b28cc + 10;
        ExceptionList = local_c;
        return;
      }
      if ((*piVar5 == 1) && (piVar5[1] == 0x20)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      FUN_00619730();
      if (bVar1) {
        *(int *)(*(int *)((int)this + 0x168) + 0x5c) = DAT_006b28cc + 10;
        ExceptionList = local_c;
        return;
      }
      if ((*piVar5 == 1) && (piVar5[1] == 0x11)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      FUN_00619730();
      if (bVar1) {
LAB_004c1f4e:
        *(int *)(*(int *)((int)this + 0x168) + 4) = DAT_006b28cc + 10;
        ExceptionList = local_c;
        return;
      }
      if ((*piVar5 == 1) && (piVar5[1] == 0x17)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_004c2230((int)this);
        ExceptionList = local_c;
        return;
      }
      if ((*piVar5 == 1) && (piVar5[1] == 0x21)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      FUN_00619730();
      if (bVar1) goto LAB_004c1f4e;
      local_4 = 9;
      if ((*piVar5 == 1) && (piVar5[1] == 0x24)) {
LAB_004c1fb6:
        bVar1 = true;
      }
      else {
        if ((*piVar5 == 1) && (piVar5[1] == 0x28)) {
          bVar1 = true;
        }
        else {
          bVar1 = false;
        }
        FUN_00619730();
        if (bVar1) goto LAB_004c1fb6;
        bVar1 = false;
      }
      local_4 = 0xffffffff;
      FUN_00619730();
      piVar2 = param_1;
      if (bVar1) {
        piVar5 = (int *)0x0;
        iVar4 = (**(code **)(*param_1 + 0x28))();
        this_00 = (void *)thunk_FUN_005f5060(iVar4);
        if (this_00 != (void *)0x0) {
          puVar3 = FUN_00403040(this_00,&param_1);
          local_4 = 10;
          if ((*puVar3 >> 0x18 < 0xa0) || (0xaf < *puVar3 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          local_4 = 0xffffffff;
          FUN_00619730();
          if (bVar1) {
            puVar3 = FUN_00403040(this_00,&param_1);
            local_4 = 0xb;
            piVar5 = FUN_004f2d10(1,puVar3);
            local_4 = 0xffffffff;
            FUN_00619730();
          }
        }
        puVar3 = (uint *)(**(code **)(*piVar2 + 0x30))(&param_1);
        local_4 = 0xc;
        if ((*puVar3 >> 0x18 < 0x90) || (0x97 < *puVar3 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        if (bVar1) {
          *(int *)(*(int *)((int)this + 0x168) + 8) = DAT_006b28cc + 10;
          ExceptionList = local_c;
          return;
        }
        if (piVar5 != (int *)0x0) {
          puVar3 = FUN_004c27b0(piVar5,&param_1);
          local_4 = 0xd;
          if ((*puVar3 >> 0x18 < 0x90) || (0x97 < *puVar3 >> 0x18)) {
            bVar1 = false;
          }
          else {
            bVar1 = true;
          }
          FUN_00619730();
          local_4 = 0xffffffff;
          FUN_00619730();
          if (bVar1) {
            *(int *)(*(int *)((int)this + 0x168) + 8) = DAT_006b28cc + 10;
            ExceptionList = local_c;
            return;
          }
        }
        *(int *)(*(int *)((int)this + 0x168) + 0x60) = DAT_006b28cc + 10;
        ExceptionList = local_c;
        return;
      }
      if ((*piVar5 == 1) && (piVar5[1] == 1)) {
        bVar1 = true;
      }
      else {
        bVar1 = false;
      }
      FUN_00619730();
      if (bVar1) {
        FUN_004c2230((int)this);
        ExceptionList = local_c;
        return;
      }
      goto LAB_004c21a9;
    }
    puVar3 = (uint *)(**(code **)(*param_1 + 0x30))(&param_2);
    local_4 = 3;
    iVar4 = FUN_004ece60(puVar3);
    if (iVar4 == 0) {
LAB_004c1ce2:
      bVar1 = false;
    }
    else {
      puVar3 = (uint *)(**(code **)(*piVar2 + 0x30))(&param_1);
      local_4._0_1_ = 4;
      if ((*puVar3 >> 0x18 < 0x90) || (0x97 < *puVar3 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_00619730();
      if (!bVar1) goto LAB_004c1ce2;
      bVar1 = true;
    }
    local_4 = 0xffffffff;
    FUN_00619730();
    if (!bVar1) {
      ExceptionList = local_c;
      return;
    }
    puVar3 = (uint *)(**(code **)(*piVar2 + 0x30))(&param_1);
    local_4 = 5;
    piVar5 = FUN_004f3220(*(int *)((int)this + 0x14),puVar3);
    local_4 = 0xffffffff;
    FUN_00619730();
    if ((piVar5 != (int *)0x0) && ((*(byte *)(piVar5 + 0x14) & 8) != 0)) {
      *(int *)(*(int *)((int)this + 0x168) + 0x18) = DAT_006b28cc + 10;
      ExceptionList = local_c;
      return;
    }
    goto LAB_004c21a9;
  }
  puVar3 = (uint *)(**(code **)(*param_1 + 0x30))(&param_1);
  local_4 = 2;
  iVar4 = FUN_004ece60(puVar3);
  if ((iVar4 == 0) || (iVar4 = (**(code **)(*piVar2 + 0xc))(), iVar4 != 0x240)) {
LAB_004c1bbc:
    bVar1 = false;
  }
  else {
    iVar4 = (**(code **)(*piVar2 + 0x28))();
    iVar4 = thunk_FUN_005f5060(iVar4);
    if ((iVar4 != 0) || (iVar4 = thunk_FUN_005f5060((int)(piVar2 + 0x16)), iVar4 == 0))
    goto LAB_004c1bbc;
    bVar1 = true;
  }
  local_4 = 0xffffffff;
  FUN_00619730();
  if (bVar1) {
    *(int *)(*(int *)((int)this + 0x168) + 0x54) = DAT_006b28cc + 10;
    ExceptionList = local_c;
    return;
  }
LAB_004c21a9:
  thunk_FUN_004c2230((int)this);
  ExceptionList = local_c;
  return;
}

