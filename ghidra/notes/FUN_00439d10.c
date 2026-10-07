
void __thiscall FUN_00439d10(void *this,int param_1)

{
  if (*(int *)((int)this + 0x10) != 0) {
    if (param_1 == 0x110) {
      FUN_0049e360(*(void **)((int)this + 0x184),2);
    }
    else {
      if (param_1 == 0x111) {
        FUN_0049e360(*(void **)((int)this + 0x184),3);
        return;
      }
      if (param_1 == 0x112) {
        FUN_0049e360(*(void **)((int)this + 0x184),1);
        return;
      }
    }
  }
  return;
}

