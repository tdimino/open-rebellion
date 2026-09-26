
void __thiscall FUN_00612e20(void *this,uint *param_1)

{
  uint uVar1;
  uint uVar2;
  uint *puVar3;
  int local_c;
  int local_8;
  int local_4;
  
  puVar3 = param_1;
  uVar2 = *param_1;
  local_8 = 0;
  if (uVar2 != 0) {
    *param_1 = uVar2 + 4;
    param_1 = (uint *)0x0;
    local_c = 0;
    FUN_00612ca0(&local_c,(int *)puVar3,4);
    *puVar3 = uVar2;
    local_c = local_c + 8;
    *puVar3 = uVar2 + *(int *)((int)this + 0x1c);
    FUN_00612ca0(&param_1,(int *)puVar3,4);
    if (((((char)param_1 == 'd') && (param_1._1_1_ == 'a')) && (param_1._2_1_ == 't')) &&
       (param_1._3_1_ == 'a')) {
      FUN_00612ca0((undefined4 *)((int)this + 0x18),(int *)puVar3,4);
      local_8 = 1;
      *puVar3 = *puVar3 - 8;
    }
    else {
      local_4 = 0;
      uVar1 = local_c + uVar2;
      *puVar3 = uVar2 + 0xc;
      *(undefined4 *)((int)this + 0x1c) = 0xc;
      uVar2 = *puVar3;
      while (uVar2 < uVar1) {
        FUN_00612ca0(&param_1,(int *)puVar3,4);
        if ((((char)param_1 == 'd') && (param_1._1_1_ == 'a')) &&
           ((param_1._2_1_ == 't' && (param_1._3_1_ == 'a')))) {
          FUN_00612ca0((undefined4 *)((int)this + 0x18),(int *)puVar3,4);
          local_8 = 1;
          *puVar3 = *puVar3 - 8;
          break;
        }
        FUN_00612ca0(&local_4,(int *)puVar3,4);
        *puVar3 = *puVar3 + local_4;
        *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + local_4 + 4;
        uVar2 = *puVar3;
      }
    }
  }
  if (local_8 != 1) {
    *(undefined4 *)((int)this + 0x1c) = 0;
    *(undefined4 *)((int)this + 0x18) = 0;
  }
  return;
}

