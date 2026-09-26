
void __thiscall FUN_0045f100(void *this,uint param_1,uint param_2)

{
  bool bVar1;
  uint uVar2;
  int *this_00;
  void *pvVar3;
  undefined4 *puVar4;
  char *pcVar5;
  int iVar6;
  undefined4 local_54;
  undefined4 local_50;
  undefined4 local_4c;
  undefined4 local_48;
  void *local_44;
  undefined4 local_40;
  uint local_3c;
  uint uStack_38;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  tagRECT tStack_1c;
  void *local_c;
  undefined1 *puStack_8;
  uint local_4;
  
  uVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630ee2;
  local_c = ExceptionList;
  bVar1 = false;
  local_54 = 0;
  if (*(uint *)((int)this + 0x118) == param_1) {
    return;
  }
  if ((*(int *)((int)this + 0x114) == 2) && (param_2 == 0)) {
    return;
  }
  if ((param_1 == 0x6f) || (param_2 != 0)) {
    ExceptionList = &local_c;
    FUN_00608280(*(void **)((int)this + 300),*(undefined4 *)(*(int *)((int)this + 0x160) + 0x474));
    FUN_0060a280(*(void **)((int)this + 300));
    if ((uVar2 != 0) && (*(int *)((int)this + 0x114) != 2)) {
      *(undefined4 *)((int)this + 0x148) = 0;
    }
    goto LAB_0045f3fc;
  }
  ExceptionList = &local_c;
  FUN_00520570(&local_3c);
  local_4 = uVar2;
  uVar2 = local_4;
  local_4._1_3_ = 0;
  switch(param_1) {
  case 0x70:
    local_44 = (void *)0x90;
    local_40 = 0x98;
    local_4._0_1_ = 1;
    FUN_00520580(&local_3c,&local_44);
    local_4 = (uint)local_4._1_3_ << 8;
    break;
  case 0x71:
    local_4c = 0x14;
    local_48 = 0x20;
    local_4._0_1_ = 2;
    FUN_00520580(&local_3c,&local_4c);
    local_4 = (uint)local_4._1_3_ << 8;
    break;
  case 0x72:
    local_54 = 0x20;
    local_50 = 0x30;
    local_4._0_1_ = 3;
    FUN_00520580(&local_3c,&local_54);
    local_4 = (uint)local_4._1_3_ << 8;
    break;
  case 0x73:
    local_34 = 0x40;
    local_30 = 0x80;
    local_4._0_1_ = 4;
    FUN_00520580(&local_3c,&local_34);
    local_4 = (uint)local_4._1_3_ << 8;
    break;
  case 0x74:
    local_2c = 0x10;
    local_28 = 0x14;
    local_4._0_1_ = 5;
    FUN_00520580(&local_3c,&local_2c);
    local_4 = (uint)local_4._1_3_ << 8;
    break;
  case 0x75:
    local_24 = 0x30;
    local_20 = 0x40;
    local_4._0_1_ = 6;
    FUN_00520580(&local_3c,&local_24);
    local_4 = (uint)local_4._1_3_ << 8;
    break;
  default:
    goto switchD_0045f17f_default;
  }
  FUN_00619730();
  uVar2 = local_4;
switchD_0045f17f_default:
  local_4 = uVar2;
  this_00 = (int *)(**(code **)(**(int **)(*(int *)((int)this + 0x160) + 0x474) + 8))();
  FUN_005f5b20(*(void **)((int)this + 0x134));
  while (this_00 != (int *)0x0) {
    FUN_0042d170(this_00,&param_2);
    local_4._0_1_ = 7;
    if ((local_3c <= param_2 >> 0x18) && (param_2 >> 0x18 < uStack_38)) {
      pvVar3 = (void *)FUN_00618b70(0x74);
      local_4._0_1_ = 8;
      local_44 = pvVar3;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        puVar4 = FUN_0042d170(this_00,&local_4c);
        bVar1 = true;
        local_54 = 1;
        local_4 = CONCAT31(local_4._1_3_,9);
        pvVar3 = FUN_00442130(pvVar3,this_00[3],puVar4);
      }
      local_4 = 8;
      if (bVar1) {
        bVar1 = false;
        FUN_00619730();
      }
      local_4._0_1_ = 7;
      pcVar5 = (char *)FUN_00583c40((int)(this_00 + 5));
      FUN_005f35e0((void *)((int)pvVar3 + 0x14),pcVar5);
      FUN_005f59f0(*(void **)((int)this + 0x134),(int)pvVar3);
    }
    this_00 = (int *)(**(code **)(*this_00 + 0xc))();
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_00619730();
  }
  FUN_00608280(*(void **)((int)this + 300),*(undefined4 *)((int)this + 0x134));
  FUN_0060a280(*(void **)((int)this + 300));
  local_4 = 0xffffffff;
  FUN_00619730();
LAB_0045f3fc:
  *(uint *)((int)this + 0x118) = param_1;
  iVar6 = FUN_00604500((void *)(*(int *)((int)this + 0x124) + 0x6c),param_1);
  FUN_00601aa0(*(void **)((int)this + 0x130),*(char **)(iVar6 + 0x84));
  SetRect(&tStack_1c,0x28,0x77,0x143,0x8a);
  InvalidateRect(*(HWND *)((int)this + 0x18),&tStack_1c,0);
  ExceptionList = local_c;
  return;
}

