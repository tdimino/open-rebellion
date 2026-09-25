
void __thiscall FUN_0059bd30(void *this,undefined4 *param_1,uint *param_2)

{
  uint *puVar1;
  void *pvVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00651926;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pvVar2 = (void *)FUN_00618b70(0x114);
  puVar1 = param_2;
  local_4 = 0;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00602150(pvVar2,*param_1,0x260,0x135,0x14,0x14,this,0xad18,0x40e,0x40f,param_2,0,0)
    ;
  }
  local_4 = 0xffffffff;
  if (pvVar2 != (void *)0x0) {
    param_2 = (uint *)CONCAT22(DAT_0065d428,0xadaa);
    FUN_005ef240(this,pvVar2,0,(int)&param_2);
  }
  param_2 = (uint *)FUN_00618b70(0x114);
  local_4 = 1;
  if (param_2 == (uint *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_00602150(param_2,*param_1,0x231,0x134,0x1c,0x15,this,0xad22,0x424,0x425,puVar1,0,0)
    ;
  }
  local_4 = 0xffffffff;
  if (pvVar2 != (void *)0x0) {
    FUN_00603150(pvVar2,4,0x425);
    FUN_006030c0(pvVar2,4);
    param_2 = (uint *)CONCAT22(DAT_0065d428,0xadb8);
    FUN_005ef240(this,pvVar2,0,(int)&param_2);
  }
  ExceptionList = local_c;
  return;
}

