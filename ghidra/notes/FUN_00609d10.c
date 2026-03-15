
void __thiscall FUN_00609d10(void *this,undefined4 param_1)

{
  bool bVar1;
  int *piVar2;
  
  piVar2 = (int *)(**(code **)(**(int **)((int)this + 0xa0) + 8))();
  bVar1 = false;
joined_r0x00609d25:
  do {
    if ((piVar2 == (int *)0x0) || (bVar1)) {
      return;
    }
    if ((piVar2[10] <= (int)(short)param_1) &&
       ((int)(short)param_1 <= *(int *)((int)this + 0xcc) + piVar2[10])) {
      if ((piVar2[0xb] <= (int)param_1._2_2_) &&
         ((int)param_1._2_2_ <= *(int *)((int)this + 0xd0) + piVar2[0xb])) {
        bVar1 = true;
        goto joined_r0x00609d25;
      }
    }
    piVar2 = (int *)(**(code **)(*piVar2 + 0xc))();
  } while( true );
}

