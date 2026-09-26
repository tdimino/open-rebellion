
uint __thiscall FUN_004f6fd0(void *this,void *param_1,void *param_2)

{
  void *pvVar1;
  bool bVar2;
  undefined3 extraout_var;
  undefined3 extraout_var_00;
  uint uVar3;
  undefined3 extraout_var_01;
  void *pvVar4;
  void *pvVar5;
  
  uVar3 = 0;
  if (param_1 != (void *)0x0) {
    bVar2 = FUN_0053a000((int)this);
    uVar3 = 0;
    if (CONCAT31(extraout_var,bVar2) != 0) {
      bVar2 = FUN_0053a000((int)param_1);
      uVar3 = 0;
      if (CONCAT31(extraout_var_00,bVar2) != 0) {
        uVar3 = FUN_0053a030(param_1,this);
        if ((uVar3 != 0) && (pvVar1 = *(void **)((int)this + 0x1c), param_1 != pvVar1)) {
          bVar2 = FUN_004fc600(param_1,this,param_2);
          uVar3 = CONCAT31(extraout_var_01,bVar2);
          if (uVar3 != 0) {
            pvVar5 = param_2;
            pvVar4 = (void *)FUN_00539fd0(this,1);
            FUN_004f9d40(pvVar4,pvVar5);
            pvVar5 = param_2;
            pvVar4 = (void *)FUN_00539fd0(this,2);
            FUN_004f9d40(pvVar4,pvVar5);
            (**(code **)(*(int *)this + 0xf4))(pvVar1,param_1,param_2);
          }
        }
      }
    }
  }
  return uVar3;
}

