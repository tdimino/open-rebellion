
undefined4 __thiscall FUN_005690b0(void *this,uint *param_1)

{
  uint uVar1;
  uint uVar2;
  
  uVar1 = *(uint *)this;
  uVar2 = *param_1;
  if (((uVar2 <= uVar1) && ((uVar1 != uVar2 || (param_1[1] <= *(uint *)((int)this + 4))))) &&
     ((uVar1 != uVar2 ||
      (((*(uint *)((int)this + 4) != param_1[1] || ((int)param_1[2] <= *(int *)((int)this + 8))) &&
       ((uVar1 != uVar2 ||
        ((((*(uint *)((int)this + 4) != param_1[1] || (*(uint *)((int)this + 8) != param_1[2])) ||
          (param_1[3] <= *(uint *)((int)this + 0xc))) &&
         (((uVar1 != uVar2 || (*(uint *)((int)this + 4) != param_1[1])) ||
          ((*(uint *)((int)this + 8) != param_1[2] ||
           ((*(uint *)((int)this + 0xc) != param_1[3] ||
            ((int)param_1[4] <= *(int *)((int)this + 0x10))))))))))))))))) {
    return 0;
  }
  return 1;
}

