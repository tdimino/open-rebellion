
void __thiscall FUN_0051c790(void *this,void *param_1)

{
  if (this != param_1) {
    *(undefined4 *)((int)this + 4) = *(undefined4 *)((int)param_1 + 4);
    *(undefined4 *)((int)this + 8) = *(undefined4 *)((int)param_1 + 8);
    *(undefined4 *)((int)this + 0xc) = *(undefined4 *)((int)param_1 + 0xc);
    *(undefined4 *)((int)this + 0x10) = *(undefined4 *)((int)param_1 + 0x10);
  }
  return;
}

