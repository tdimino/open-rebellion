
void __thiscall FUN_00609590(void *this,undefined4 param_1,undefined4 param_2)

{
  bool bVar1;
  short sVar2;
  int *piVar3;
  short sVar4;
  
  bVar1 = false;
  piVar3 = (int *)(**(code **)(**(int **)((int)this + 0xa0) + 8))();
  sVar2 = param_1._2_2_;
  sVar4 = (short)param_1;
  if ((short)param_2 < (short)param_1) {
    sVar4 = (short)param_2;
    param_2._0_2_ = (short)param_1;
  }
  if (param_2._2_2_ < param_1._2_2_) {
    param_1._2_2_ = param_2._2_2_;
    param_2._2_2_ = sVar2;
  }
  while (piVar3 != (int *)0x0) {
    if ((((piVar3[10] <= (int)(short)param_2) &&
         ((int)sVar4 <= *(int *)((int)this + 0xcc) + piVar3[10])) &&
        (piVar3[0xb] <= (int)param_2._2_2_)) &&
       ((int)param_1._2_2_ <= *(int *)((int)this + 0xd0) + piVar3[0xb])) {
      bVar1 = true;
      piVar3[0xf] = piVar3[0xf] | 1;
    }
    piVar3 = (int *)(**(code **)(*piVar3 + 0xc))();
  }
  if (bVar1) {
    FUN_00609a00(this);
    if (*(HWND *)((int)this + 0x18) != (HWND)0x0) {
      InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
    }
  }
  return;
}

