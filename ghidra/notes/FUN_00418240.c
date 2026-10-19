
void __thiscall FUN_00418240(void *this,uint param_1)

{
  *(int *)((int)this + (param_1 & 0xffff) * 4 + 0x364) =
       *(int *)((int)this + (param_1 & 0xffff) * 4 + 0x364) + 1;
  FUN_004311d0((int)this + 0x2c);
  return;
}

