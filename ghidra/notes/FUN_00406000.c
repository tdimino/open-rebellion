
void __thiscall FUN_00406000(void *this,undefined4 param_1)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  int iVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined3 extraout_var_04;
  undefined3 extraout_var_05;
  undefined3 extraout_var_06;
  undefined3 extraout_var_07;
  undefined3 extraout_var_08;
  int iVar5;
  int iVar6;
  undefined4 local_2c [2];
  undefined4 local_24 [2];
  tagRECT local_1c;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006298b0;
  local_c = ExceptionList;
  switch(param_1) {
  case 0x65:
    ExceptionList = &local_c;
    FUN_00404fa0((int)this);
    pvVar3 = (void *)FUN_005f5650(local_2c);
    iVar6 = *(int *)((int)this + 0xf4);
    iVar5 = *(int *)((int)this + 0xf8) + 1;
    local_4 = 0;
    iVar4 = FUN_005f56b0(pvVar3,0);
    FUN_004010a0(1,*(undefined4 *)((int)this + 0xf0),*(undefined4 *)((int)this + 0xec),iVar4,iVar6,
                 iVar5);
    local_4 = 0xffffffff;
    FUN_005f56a0(local_2c);
    iVar6 = 1;
    goto LAB_00406180;
  case 0x66:
    ExceptionList = &local_c;
    FUN_00404fa0((int)this);
    pvVar3 = (void *)FUN_005f5650(local_24);
    iVar6 = *(int *)((int)this + 0xf4);
    iVar5 = *(int *)((int)this + 0xf8) + 4;
    local_4 = 1;
    iVar4 = FUN_005f56b0(pvVar3,0);
    FUN_004010a0(2,*(undefined4 *)((int)this + 0xf0),*(undefined4 *)((int)this + 0xec),iVar4,iVar6,
                 iVar5);
    local_4 = 0xffffffff;
    FUN_005f56a0(local_24);
    iVar6 = 2;
LAB_00406180:
    FUN_0041daf0(iVar6,1);
    FUN_00610c00(0);
    *(undefined4 *)((int)this + 0x100) = 1;
    FUN_00604460(&DAT_006be4c8,1);
    FUN_00604460(&DAT_006be4c8,8);
    FUN_00401040(7);
    ExceptionList = local_c;
    return;
  case 0x67:
    ExceptionList = &local_c;
    *(undefined4 *)((int)this + 0xf0) = 2;
    FUN_00401040(4);
    ExceptionList = local_c;
    return;
  case 0x68:
    ExceptionList = &local_c;
    FUN_00404fa0((int)this);
    FUN_00401040(0xb);
    ExceptionList = local_c;
    return;
  case 0x69:
    ExceptionList = &local_c;
    FUN_00401040(0xd);
    ExceptionList = local_c;
    return;
  case 0x6a:
    ExceptionList = &local_c;
    SetRect(&local_1c,0x11f,0x122,0x184,0x13b);
    if (*(int *)((int)this + 0xac) != 0) {
      uVar2 = *(uint *)(*(int *)((int)this + 0xac) + 0x134) & 3;
      if (uVar2 == 0) {
        *(undefined4 *)((int)this + 0xec) = 3;
      }
      else if (uVar2 == 1) {
        *(undefined4 *)((int)this + 0xec) = 2;
      }
      else if (uVar2 == 2) {
        *(undefined4 *)((int)this + 0xec) = 1;
      }
      else {
        *(undefined4 *)((int)this + 0xec) = 0;
      }
    }
    InvalidateRect(*(HWND *)((int)this + 0x18),&local_1c,1);
    ExceptionList = local_c;
    return;
  case 0x6e:
    ExceptionList = &local_c;
    bVar1 = FUN_00603120(*(void **)((int)this + 0xb8),4);
    if (CONCAT31(extraout_var,bVar1) == 0) {
      FUN_006030c0(*(void **)((int)this + 0xb8),4);
    }
    else {
      if ((*(void **)((int)this + 0xbc) != (void *)0x0) &&
         (bVar1 = FUN_00603120(*(void **)((int)this + 0xbc),4), CONCAT31(extraout_var_00,bVar1) != 0
         )) {
        FUN_006030f0(*(void **)((int)this + 0xbc),4);
      }
      if ((*(void **)((int)this + 0xc0) != (void *)0x0) &&
         (bVar1 = FUN_00603120(*(void **)((int)this + 0xc0),4), CONCAT31(extraout_var_01,bVar1) != 0
         )) {
        FUN_006030f0(*(void **)((int)this + 0xc0),4);
        *(undefined4 *)((int)this + 0xf8) = 0;
        ExceptionList = local_c;
        return;
      }
    }
    *(undefined4 *)((int)this + 0xf8) = 0;
    ExceptionList = local_c;
    return;
  case 0x6f:
    ExceptionList = &local_c;
    bVar1 = FUN_00603120(*(void **)((int)this + 0xbc),4);
    if (CONCAT31(extraout_var_02,bVar1) == 0) {
      FUN_006030c0(*(void **)((int)this + 0xbc),4);
    }
    else {
      if ((*(void **)((int)this + 0xb8) != (void *)0x0) &&
         (bVar1 = FUN_00603120(*(void **)((int)this + 0xb8),4), CONCAT31(extraout_var_03,bVar1) != 0
         )) {
        FUN_006030f0(*(void **)((int)this + 0xb8),4);
      }
      if ((*(void **)((int)this + 0xc0) != (void *)0x0) &&
         (bVar1 = FUN_00603120(*(void **)((int)this + 0xc0),4), CONCAT31(extraout_var_04,bVar1) != 0
         )) {
        FUN_006030f0(*(void **)((int)this + 0xc0),4);
        *(undefined4 *)((int)this + 0xf8) = 1;
        ExceptionList = local_c;
        return;
      }
    }
    *(undefined4 *)((int)this + 0xf8) = 1;
    ExceptionList = local_c;
    return;
  case 0x70:
    ExceptionList = &local_c;
    bVar1 = FUN_00603120(*(void **)((int)this + 0xc0),4);
    if (CONCAT31(extraout_var_05,bVar1) == 0) {
      FUN_006030c0(*(void **)((int)this + 0xc0),4);
    }
    else {
      if ((*(void **)((int)this + 0xbc) != (void *)0x0) &&
         (bVar1 = FUN_00603120(*(void **)((int)this + 0xbc),4), CONCAT31(extraout_var_06,bVar1) != 0
         )) {
        FUN_006030f0(*(void **)((int)this + 0xbc),4);
      }
      if ((*(void **)((int)this + 0xb8) != (void *)0x0) &&
         (bVar1 = FUN_00603120(*(void **)((int)this + 0xb8),4), CONCAT31(extraout_var_07,bVar1) != 0
         )) {
        FUN_006030f0(*(void **)((int)this + 0xb8),4);
        *(undefined4 *)((int)this + 0xf8) = 2;
        ExceptionList = local_c;
        return;
      }
    }
    *(undefined4 *)((int)this + 0xf8) = 2;
    ExceptionList = local_c;
    return;
  case 0x71:
    ExceptionList = &local_c;
    bVar1 = FUN_00603120(*(void **)((int)this + 0xa8),4);
    if (CONCAT31(extraout_var_08,bVar1) == 0) {
      pvVar3 = *(void **)((int)this + 0xa0);
      *(undefined4 *)((int)this + 0xf4) = 1;
      if (pvVar3 != (void *)0x0) {
        param_1 = CONCAT22(DAT_0065d420,0x1060);
        goto LAB_0040646e;
      }
    }
    else {
      pvVar3 = *(void **)((int)this + 0xa0);
      *(undefined4 *)((int)this + 0xf4) = 2;
      if (pvVar3 != (void *)0x0) {
        param_1 = CONCAT22(DAT_0065d420,0x1061);
LAB_0040646e:
        FUN_006019a0(pvVar3,&param_1);
        FUN_00601dc0(*(void **)((int)this + 0xa0),*(HWND *)((int)this + 0x18),1);
        ExceptionList = local_c;
        return;
      }
    }
    break;
  case 0x73:
    ExceptionList = &local_c;
    FUN_00610c00(0);
    *(undefined4 *)((int)this + 0x100) = 1;
    *(undefined4 *)((int)this + 0x104) = 1;
    (**(code **)(*(int *)this + 8))();
    FUN_004019a0(0x6e,0,&LAB_00406520);
  }
  ExceptionList = local_c;
  return;
}

