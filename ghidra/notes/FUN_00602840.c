
void __thiscall FUN_00602840(void *this,ushort param_1)

{
  undefined4 uVar1;
  
  *(ushort *)((int)this + 0xdc) = param_1;
  *(uint *)((int)this + 0xe4) = (uint)param_1;
  *(uint *)((int)this + 0xf0) = *(uint *)((int)this + 0xf0) & 0xfffffff9 | 1;
  uVar1 = FUN_005feff0(*(int *)((int)this + 0x1c));
  *(undefined4 *)((int)this + 0xe8) = uVar1;
  return;
}

