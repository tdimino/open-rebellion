
undefined4 __thiscall FUN_00612f80(void *this,int *param_1)

{
  int *this_00;
  int iVar1;
  int local_4;
  
  this_00 = param_1;
  if (*(int *)(*(int *)(*(int *)(*param_1 + 4) + 4 + (int)param_1) + 0x4c) == -1) {
    return 0xe00b;
  }
  param_1 = (int *)0x0;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x1c) = 0;
  FUN_006173b0(this_00,&param_1,4);
  if (((((char)param_1 == 'R') && (param_1._1_1_ == 'I')) && (param_1._2_1_ == 'F')) &&
     (param_1._3_1_ == 'F')) {
    FUN_006173b0(this_00,&local_4,4);
    *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + 8;
    FUN_006173b0(this_00,&param_1,4);
    if ((((char)param_1 == 'W') && (param_1._1_1_ == 'A')) &&
       ((param_1._2_1_ == 'V' && (param_1._3_1_ == 'E')))) {
      *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + 4;
      FUN_006173b0(this_00,&param_1,4);
      if ((((char)param_1 == 'f') && (param_1._1_1_ == 'm')) &&
         ((param_1._2_1_ == 't' && (param_1._3_1_ == ' ')))) {
        FUN_006173b0(this_00,&local_4,4);
        *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + local_4 + 8;
        FUN_006173b0(this_00,(short *)((int)this + 4),2);
        FUN_006173b0(this_00,(int)this + 6,2);
        FUN_006173b0(this_00,(int)this + 8,4);
        FUN_006173b0(this_00,(int)this + 0xc,4);
        FUN_006173b0(this_00,(int)this + 0x10,2);
        FUN_006173b0(this_00,(int)this + 0x12,2);
        if (*(short *)((int)this + 4) == 1) {
          iVar1 = FUN_00612ce0(this,this_00,0);
          if (iVar1 != 0) {
            return 0;
          }
        }
        return 0xe007;
      }
      return 0xe006;
    }
  }
  return 0xe003;
}

