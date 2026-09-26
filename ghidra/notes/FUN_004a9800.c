
void __thiscall FUN_004a9800(void *this,HDC param_1)

{
  void *this_00;
  int iVar1;
  int iVar2;
  
  if (*(int *)(*(int *)((int)this + 0x14c) + 0xb8) == *(int *)((int)this + 0x18)) {
    iVar2 = *(int *)((int)this + 300);
    iVar1 = *(int *)((int)this + 0x128);
    this_00 = *(void **)((int)this + 0x17c);
  }
  else {
    iVar2 = *(int *)((int)this + 300);
    iVar1 = *(int *)((int)this + 0x128);
    this_00 = *(void **)((int)this + 0x180);
  }
  FUN_005fc140(this_00,param_1,'@',0xcc0020,iVar1,iVar2,0,0,0,0);
  FUN_00601ce0(*(void **)((int)this + 0x16c),param_1);
  FUN_00601ce0(*(void **)((int)this + 0x124),param_1);
  FUN_00601ce0(*(void **)((int)this + 0x170),param_1);
  return;
}

