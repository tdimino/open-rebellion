
void __thiscall
FUN_0058be10(void *this,void *param_1,undefined4 param_2,undefined4 param_3,uint *param_4,
            void *param_5)

{
  uint uVar1;
  
  *param_4 = 0;
  if ((param_1 != (void *)0x0) &&
     (uVar1 = (uint)(*(int *)((int)this + 8) == *(int *)((int)this + 0xc)), *param_4 = uVar1,
     uVar1 == 0)) {
    *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    FUN_004f6fd0(param_1,*(void **)(*(int *)((int)this + 4) + 8),param_5);
  }
  return;
}

