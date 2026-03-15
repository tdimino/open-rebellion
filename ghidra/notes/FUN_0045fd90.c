
uint * __thiscall FUN_0045fd90(void *this,uint *param_1,uint *param_2)

{
  bool bVar1;
  int *this_00;
  
  if ((*param_2 >> 0x18 < 0xa0) || (0xaf < *param_2 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (!bVar1) {
    FUN_004f26d0(param_1,param_2);
    return param_1;
  }
  this_00 = FUN_004f2f60(*(int *)(*(int *)((int)this + 0x160) + 0x9c),param_2);
  if (this_00 == (int *)0x0) {
    FUN_004ece30(param_1);
    return param_1;
  }
  if (this_00[0x16] != 0) {
    FUN_0040d760(this_00,param_1);
    return param_1;
  }
  FUN_004025b0((void *)this_00[7],param_1);
  return param_1;
}

