
void * __thiscall FUN_006172b0(void *this,int param_1,int param_2)

{
  if ((*(int *)((int)this + 0x4c) != -1) && (*(int *)((int)this + 0x14) != 0)) {
    return (void *)0x0;
  }
  if ((param_1 == 0) || (param_2 < 1)) {
    *(undefined4 *)((int)this + 8) = 1;
  }
  else {
    if (*(int *)((int)this + 0x30) < 0) {
      FUN_00615ee0((LPCRITICAL_SECTION)((int)this + 0x34));
    }
    FUN_00618020(this,param_1,param_2 + param_1,0);
    if (*(int *)((int)this + 0x30) < 0) {
      FUN_00615ef0((LPCRITICAL_SECTION)((int)this + 0x34));
      return this;
    }
  }
  return this;
}

