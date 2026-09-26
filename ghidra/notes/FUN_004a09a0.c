
void * __thiscall FUN_004a09a0(void *this,void *param_1,LONG param_2,LONG param_3)

{
  POINT pt;
  LONG LVar1;
  int iVar2;
  int iVar3;
  BOOL BVar4;
  undefined4 *puVar5;
  void *this_00;
  undefined4 local_24;
  undefined4 local_20;
  tagRECT local_1c;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006375e7;
  pvStack_c = ExceptionList;
  local_20 = 0;
  ExceptionList = &pvStack_c;
  FUN_004f26d0(&local_24,(undefined4 *)((int)this + 0x144));
  LVar1 = param_3;
  local_4 = 1;
  if (*(int **)((int)this + 0x170) != (int *)0x0) {
    iVar2 = FUN_005fc0f0(*(int **)((int)this + 0x170));
    iVar2 = (0x32 - iVar2) / 2 + 0x25;
    iVar3 = FUN_005fc0e0(*(int **)((int)this + 0x170));
    SetRect(&local_1c,0x6c,0x25,(0x7a - iVar3) / 2 + 0x6c,iVar2);
    pt.y = LVar1;
    pt.x = param_2;
    BVar4 = PtInRect(&local_1c,pt);
    if (BVar4 != 0) {
      FUN_004f26d0(&local_24,(undefined4 *)((int)this + 0x1c8));
    }
  }
  if (*(void **)((int)this + 0x1c0) != (void *)0x0) {
    param_3 = CONCAT22((short)LVar1 + -0x91,(short)param_2 + -0x6b);
    puVar5 = (undefined4 *)FUN_00609d80(*(void **)((int)this + 0x1c0),&param_3,param_3);
    this_00 = (void *)FUN_00609d10(*(void **)((int)this + 0x1c0),*puVar5);
    if (this_00 != (void *)0x0) {
      puVar5 = FUN_0042d170(this_00,&param_3);
      local_4._0_1_ = 2;
      FUN_004f26d0(&local_24,puVar5);
      local_4 = CONCAT31(local_4._1_3_,1);
      FUN_00619730();
    }
  }
  FUN_004f26d0(param_1,&local_24);
  local_20 = 1;
  local_4 = local_4 & 0xffffff00;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return param_1;
}

