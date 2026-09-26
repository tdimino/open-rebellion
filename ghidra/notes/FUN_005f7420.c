
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __thiscall FUN_005f7420(void *this,int param_1)

{
  undefined4 uVar1;
  float10 fVar2;
  longlong lVar3;
  
  if ((int)(param_1 * 3 + (param_1 * 3 >> 0x1f & 3U)) >> 2 < -0x2580) {
    uVar1 = 0;
  }
  else {
    fVar2 = (float10)FUN_006197b0();
    FUN_00619610(SUB84((double)(fVar2 * (float10)_DAT_0066ddf0),0),
                 (uint)((ulonglong)(double)(fVar2 * (float10)_DAT_0066ddf0) >> 0x20));
    lVar3 = __ftol();
    uVar1 = (undefined4)lVar3;
  }
  if (*(int *)((int)this + 0x1ac) != 0) {
    _SmackVolumePan_16(*(int *)((int)this + 0x1ac),0xfe000,uVar1,0x8000);
  }
  if (*(int *)((int)this + 0x1c0) == 0) {
    *(undefined4 *)((int)this + 0x1c0) = 1;
  }
  return;
}

