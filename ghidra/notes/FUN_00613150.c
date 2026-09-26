
undefined4 __thiscall FUN_00613150(void *this,uint param_1)

{
  uint uVar1;
  int iVar2;
  uint local_8;
  int local_4;
  
  uVar1 = param_1;
  if (param_1 == 0) {
    return 0xe008;
  }
  local_8 = param_1;
  param_1 = 0;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x1c) = 0;
  FUN_00612ca0(&param_1,(int *)&local_8,4);
  if (((((char)param_1 == 'R') && (param_1._1_1_ == 'I')) && (param_1._2_1_ == 'F')) &&
     (param_1._3_1_ == 'F')) {
    FUN_00612ca0(&local_4,(int *)&local_8,4);
    *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + 8;
    FUN_00612ca0(&param_1,(int *)&local_8,4);
    if ((((char)param_1 == 'W') && (param_1._1_1_ == 'A')) &&
       ((param_1._2_1_ == 'V' && (param_1._3_1_ == 'E')))) {
      *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + 4;
      FUN_00612ca0(&param_1,(int *)&local_8,4);
      if ((((char)param_1 == 'f') && (param_1._1_1_ == 'm')) &&
         ((param_1._2_1_ == 't' && (param_1._3_1_ == ' ')))) {
        FUN_00612ca0(&local_4,(int *)&local_8,4);
        *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + local_4 + 8;
        FUN_00612ca0((undefined4 *)((int)this + 4),(int *)&local_8,2);
        FUN_00612ca0((undefined4 *)((int)this + 6),(int *)&local_8,2);
        FUN_00612ca0((undefined4 *)((int)this + 8),(int *)&local_8,4);
        FUN_00612ca0((undefined4 *)((int)this + 0xc),(int *)&local_8,4);
        FUN_00612ca0((undefined4 *)((int)this + 0x10),(int *)&local_8,2);
        FUN_00612ca0((undefined4 *)((int)this + 0x12),(int *)&local_8,2);
        local_8 = uVar1;
        if (*(short *)((int)this + 4) == 1) {
          iVar2 = FUN_00612e20(this,&local_8);
          if (iVar2 != 0) {
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

