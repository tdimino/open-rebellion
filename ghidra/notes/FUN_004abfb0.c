
void * __thiscall
FUN_004abfb0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,
            int param_5,int param_6,int param_7,int param_8,uint param_9,uint param_10,uint param_11
            ,uint param_12,undefined4 param_13,void *param_14,void *param_15,void *param_16)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006384c3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ab560(this,param_3,param_4,param_5,param_6,param_7,param_8,param_9,param_10,param_11,
               param_12,param_13,param_14,param_15,param_16);
  local_4 = 0;
  FUN_0051c900((undefined4 *)((int)this + 0x78),0,0);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_004aa910((undefined4 *)((int)this + 0x80));
  *(undefined ***)this = &PTR_FUN_0065bf68;
  *(undefined4 *)((int)this + 0x78) = param_1;
  *(undefined4 *)((int)this + 0x7c) = param_2;
  ExceptionList = local_c;
  return this;
}

