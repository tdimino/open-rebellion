
undefined4 * __thiscall
FUN_0041a400(void *this,undefined4 *param_1,undefined4 *param_2,uint param_3,uint param_4,
            uint param_5,uint param_6,uint param_7,uint param_8,undefined4 *param_9)

{
  void *this_00;
  undefined4 *puVar1;
  
  puVar1 = (undefined4 *)0x0;
  FUN_004f26d0(&stack0xfffffff0,param_1);
  this_00 = FUN_0041bd90((void *)((int)this + 0x44));
  if (this_00 != (void *)0x0) {
    puVar1 = FUN_00431c00(this_00,param_2,param_3,param_4,param_5,param_6,param_7,param_8,param_9);
  }
  return puVar1;
}

