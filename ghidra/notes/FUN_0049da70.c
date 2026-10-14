
undefined4 * __thiscall
FUN_0049da70(void *this,undefined4 *param_1,undefined4 *param_2,uint param_3,uint param_4,
            uint param_5,uint param_6,undefined4 *param_7)

{
  void *this_00;
  undefined4 *puVar1;
  
  puVar1 = (undefined4 *)0x0;
  FUN_004f26d0(&stack0xfffffff0,param_1);
  this_00 = FUN_0049dca0((void *)((int)this + 0x20));
  if (this_00 != (void *)0x0) {
    puVar1 = FUN_004c6460(this_00,param_2,param_3,param_4,param_5,param_6,param_7);
  }
  return puVar1;
}

