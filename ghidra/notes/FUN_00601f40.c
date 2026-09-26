
void * __thiscall FUN_00601f40(void *this,UINT param_1,UINT param_2,uint param_3)

{
  LPCRITICAL_SECTION lpCriticalSection;
  MMRESULT MVar1;
  
  lpCriticalSection = (LPCRITICAL_SECTION)((int)this + 0x20);
  *(undefined ***)this = &PTR_FUN_0066df50;
  InitializeCriticalSection(lpCriticalSection);
  EnterCriticalSection(lpCriticalSection);
  *(undefined4 *)((int)this + 4) = 1;
  *(undefined4 *)((int)this + 0x18) = 0;
  *(undefined4 *)((int)this + 8) = 0;
  *(UINT *)((int)this + 0x10) = param_1;
  *(UINT *)((int)this + 0x14) = param_2;
  *(uint *)((int)this + 0xc) = param_3;
  *(undefined4 *)((int)this + 0x1c) = 0;
  if (param_3 <= DAT_006ac7d4) {
    param_3 = DAT_006ac7d4;
  }
  *(uint *)((int)this + 0xc) = param_3;
  if ((param_1 != 0) && (DAT_006be5b8 != 0)) {
    MVar1 = timeSetEvent(param_1,param_3,fptc_006020d0,(DWORD_PTR)this,param_2);
    *(MMRESULT *)((int)this + 8) = MVar1;
    if (MVar1 != 0) {
      *(undefined4 *)((int)this + 0x18) = 1;
    }
  }
  *(undefined4 *)((int)this + 4) = 0;
  LeaveCriticalSection(lpCriticalSection);
  return this;
}

