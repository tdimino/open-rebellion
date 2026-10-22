
void __thiscall FUN_005ef240(void *this,void *param_1,int param_2,int param_3)

{
  if (param_1 != (void *)0x0) {
    FUN_005ffce0(param_1,0);
    if (param_2 != 0) {
      FUN_00602520(param_1,param_2);
    }
    if (param_3 != 0) {
      FUN_00600a40(this,param_1,param_3);
    }
    FUN_005f4f10((void *)((int)this + 0x6c),(int)param_1);
  }
  return;
}

