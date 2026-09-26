
void __thiscall FUN_004aba60(void *this,HDC param_1,int param_2)

{
  DWORD DVar1;
  DWORD DVar2;
  void *this_00;
  int iVar3;
  int iVar4;
  
  if (*(int *)((int)this + 0x58) == 0) {
    FUN_00601c90(*(void **)((int)this + 0x38),*(int *)((int)this + 0x34),(HWND)0x0);
    if (*(int **)((int)this + 0x18) == (int *)0x0) goto LAB_004abb37;
    DVar1 = FUN_005fc0f0(*(int **)((int)this + 0x18));
    DVar2 = FUN_005fc0e0(*(int **)((int)this + 0x18));
    iVar4 = *(int *)((int)this + 0x48);
    iVar3 = *(int *)((int)this + 0x44);
    this_00 = *(void **)((int)this + 0x18);
  }
  else if (param_2 == 0) {
    FUN_00601c90(*(void **)((int)this + 0x38),*(int *)((int)this + 0x30),(HWND)0x0);
    if (*(int **)((int)this + 0x14) == (int *)0x0) goto LAB_004abb37;
    DVar1 = FUN_005fc0f0(*(int **)((int)this + 0x14));
    DVar2 = FUN_005fc0e0(*(int **)((int)this + 0x14));
    iVar4 = *(int *)((int)this + 0x48);
    iVar3 = *(int *)((int)this + 0x44);
    this_00 = *(void **)((int)this + 0x14);
  }
  else {
    FUN_00601c90(*(void **)((int)this + 0x38),*(int *)((int)this + 0x2c),(HWND)0x0);
    if (*(int **)((int)this + 0x10) == (int *)0x0) goto LAB_004abb37;
    DVar1 = FUN_005fc0f0(*(int **)((int)this + 0x10));
    DVar2 = FUN_005fc0e0(*(int **)((int)this + 0x10));
    iVar4 = *(int *)((int)this + 0x48);
    iVar3 = *(int *)((int)this + 0x44);
    this_00 = *(void **)((int)this + 0x10);
  }
  FUN_005fc140(this_00,param_1,'@',0xcc0020,iVar3,iVar4,0,0,DVar2,DVar1);
LAB_004abb37:
  if ((*(int *)((int)this + 0x5c) != 0) && (*(int **)((int)this + 0x1c) != (int *)0x0)) {
    DVar1 = FUN_005fc0f0(*(int **)((int)this + 0x1c));
    DVar2 = FUN_005fc0e0(*(int **)((int)this + 0x1c));
    FUN_005fc140(*(void **)((int)this + 0x1c),param_1,'@',0xcc0020,*(int *)((int)this + 0x44),
                 *(int *)((int)this + 0x48),0,0,DVar2,DVar1);
  }
  FUN_00601ce0(*(void **)((int)this + 0x38),param_1);
  return;
}

