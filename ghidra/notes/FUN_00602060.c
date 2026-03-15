
void __thiscall FUN_00602060(void *this,UINT param_1,UINT param_2)

{
  UINT uDelay;
  MMRESULT MVar1;
  
  FUN_00602020((int)this);
  uDelay = param_1;
  if (param_1 == 0) {
    uDelay = *(UINT *)((int)this + 0x10);
  }
  *(UINT *)((int)this + 0x10) = uDelay;
  if (param_2 == 0) {
    param_2 = *(UINT *)((int)this + 0x14);
  }
  *(UINT *)((int)this + 0x14) = param_2;
  if ((DAT_006be5b8 != 0) && ((param_1 != 0 || (uDelay != 0)))) {
    MVar1 = timeSetEvent(uDelay,*(UINT *)((int)this + 0xc),fptc_006020d0,(DWORD_PTR)this,param_2);
    *(MMRESULT *)((int)this + 8) = MVar1;
    if (MVar1 != 0) {
      *(undefined4 *)((int)this + 0x18) = 1;
    }
  }
  *(undefined4 *)((int)this + 4) = 0;
  return;
}

