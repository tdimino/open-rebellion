
void __thiscall FUN_005136d0(void *this,int *param_1)

{
  int *piVar1;
  bool bVar2;
  undefined4 uVar3;
  void *pvVar4;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  int iVar5;
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006424a8;
  local_c = ExceptionList;
  param_1 = (int *)0x0;
  ExceptionList = &local_c;
  *piVar1 = 1;
  uVar3 = FUN_0051c000();
  switch(uVar3) {
  case 2:
    DAT_006b2aec = &LAB_0051c040;
    pvVar4 = (void *)FUN_004fd400(local_28);
    local_4 = 0;
    FUN_004fd490((void *)((int)this + 0x15c),pvVar4);
    local_4 = 0xffffffff;
    FUN_004fd4d0(local_28);
    goto LAB_005138d6;
  case 3:
    thunk_FUN_00567430();
    goto LAB_005138d6;
  case 4:
    thunk_FUN_0058b370();
    goto LAB_005138d6;
  case 5:
    FUN_00518dc0((int)this);
    goto LAB_005138d6;
  case 6:
    FUN_0053f3d0();
    FUN_0053f4f0();
    DAT_006b2bb0 = this;
    goto LAB_005138d6;
  case 7:
  case 0x16:
    iVar5 = FUN_0053f640();
    break;
  case 8:
    bVar2 = FUN_00519800(this,(void *)((int)this + 0x15c));
    iVar5 = CONCAT31(extraout_var,bVar2);
    break;
  case 9:
    iVar5 = FUN_00519910(this,(void *)((int)this + 0x15c));
    break;
  case 10:
    iVar5 = FUN_00519c20(this,*(int *)((int)this + 0x5c),(int)this + 0x15c);
    break;
  case 0xb:
    iVar5 = FUN_00519d00();
    break;
  case 0xc:
    iVar5 = FUN_0051a280(this,(uint *)&param_1,(void *)((int)this + 0x15c));
    goto LAB_00513826;
  case 0xd:
    iVar5 = FUN_0051a530((int)this);
    break;
  case 0xe:
    iVar5 = FUN_0051a550((int)this);
    break;
  case 0xf:
    bVar2 = FUN_0051a570(this,(uint *)&param_1,(int)this + 0x15c);
    iVar5 = CONCAT31(extraout_var_00,bVar2);
LAB_00513826:
    *piVar1 = iVar5;
    if ((iVar5 == 0) || (param_1 == (int *)0x0)) goto switchD_0051370f_default;
    goto LAB_005138d6;
  case 0x10:
    iVar5 = FUN_0051a640(this,(void *)((int)this + 0x15c));
    break;
  case 0x11:
    iVar5 = FUN_0051b750((void *)((int)this + 0x15c));
    break;
  case 0x12:
    iVar5 = FUN_0051aa50(this,(void *)((int)this + 0x15c));
    break;
  case 0x13:
    iVar5 = FUN_0051b750((void *)((int)this + 0x15c));
    break;
  case 0x14:
    bVar2 = FUN_0051b6d0((void *)((int)this + 0x15c));
    iVar5 = CONCAT31(extraout_var_01,bVar2);
    break;
  case 0x15:
    bVar2 = FUN_0051b710((void *)((int)this + 0x15c));
    iVar5 = CONCAT31(extraout_var_02,bVar2);
    break;
  case 0x17:
    iVar5 = FUN_0051b7f0((undefined4 *)((int)this + 0x15c));
    break;
  case 0x18:
    iVar5 = FUN_0051b940(this,(int)this + 0x15c);
    break;
  case 0x19:
    iVar5 = FUN_0040f340();
    break;
  default:
    goto switchD_0051370f_default;
  }
  *piVar1 = iVar5;
LAB_005138d6:
  FUN_0051c030();
switchD_0051370f_default:
  FUN_00513960();
  ExceptionList = local_c;
  return;
}

