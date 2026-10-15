
bool __thiscall FUN_005287f0(void *this,void *param_1)

{
  bool bVar1;
  
  bVar1 = (*(uint *)((int)this + 0x24) & 0x30) == 0;
  if (bVar1) {
    FUN_004f43b0(param_1,(void *)(*(int *)((int)this + 0x54) + 0x18));
  }
  return bVar1;
}

